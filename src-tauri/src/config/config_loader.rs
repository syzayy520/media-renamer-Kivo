// config_loader 模块 - 配置文件读写
// 职责：AppConfig 结构体定义、config.toml 序列化/反序列化、默认值提供
// 禁止：业务逻辑、网络请求

use crate::shared::result_types::{AppError, AppResult};
use serde::{Deserialize, Serialize};
use std::path::Path;

/// 应用配置顶层结构
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AppConfig {
    pub templates: TemplatesConfig,
    pub thresholds: ThresholdsConfig,
    pub limits: LimitsConfig,
}

/// 模板配置
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TemplatesConfig {
    pub movie: String,
    pub series: String,
    pub anime: String,
    pub special: String,
    pub extras: String,
}

/// 阈值配置
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ThresholdsConfig {
    pub confidence: u8,
}

/// 限制配置
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LimitsConfig {
    pub max_files: u32,
    pub max_path_length: usize,
}

/// 获取默认配置
pub fn default_config() -> AppConfig {
    AppConfig {
        templates: TemplatesConfig {
            movie: "{Title} ({Year}) [{Resolution} {Source} {VideoCodec} {AudioCodec}].{ext}"
                .to_string(),
            series: "{Title} - S{Season:02}E{Episode:02} - {EpisodeTitle}.{ext}".to_string(),
            anime: "{Title} - S{Season:02}E{Episode:02} [{Group}][{Resolution}].{ext}".to_string(),
            special: "{Title} - S00E{SpecialNumber:02} - {SpecialTitle}.{ext}".to_string(),
            extras: "{Title} - Extra - {ExtraType}{ExtraNumber}.{ext}".to_string(),
        },
        thresholds: ThresholdsConfig { confidence: 70 },
        limits: LimitsConfig {
            max_files: 100_000,
            max_path_length: 260,
        },
    }
}

/// 从 TOML 字符串解析配置
pub fn parse_config(toml_str: &str) -> AppResult<AppConfig> {
    toml::from_str(toml_str).map_err(|e| AppError::Config(format!("TOML parse error: {}", e)))
}

/// 将配置序列化为 TOML 字符串
pub fn serialize_config(config: &AppConfig) -> AppResult<String> {
    toml::to_string_pretty(config)
        .map_err(|e| AppError::Config(format!("TOML serialize error: {}", e)))
}

/// 从文件加载配置，文件不存在时返回默认配置
pub fn load_config(config_dir: &Path) -> AppResult<AppConfig> {
    let config_path = config_dir.join("config.toml");
    if !config_path.exists() {
        return Ok(default_config());
    }
    let content = std::fs::read_to_string(&config_path)
        .map_err(|e| AppError::Config(format!("Read config file error: {}", e)))?;
    parse_config(&content)
}

/// 保存配置到文件
pub fn save_config(config: &AppConfig, config_dir: &Path) -> AppResult<()> {
    let config_path = config_dir.join("config.toml");
    let content = serialize_config(config)?;
    std::fs::write(&config_path, content)
        .map_err(|e| AppError::Config(format!("Write config file error: {}", e)))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config_values() {
        let config = default_config();
        assert_eq!(config.thresholds.confidence, 70);
        assert_eq!(config.limits.max_files, 100_000);
        assert_eq!(config.limits.max_path_length, 260);
        assert!(config.templates.movie.contains("{Title}"));
        assert!(config.templates.series.contains("{Season:02}"));
    }

    #[test]
    fn test_parse_valid_toml() {
        let toml_str = r#"
[templates]
movie = "{Title}.{ext}"
series = "{Title} - S{Season:02}E{Episode:02}.{ext}"
anime = "{Title} - {Episode:02}.{ext}"
special = "{Title} - SP{Episode:02}.{ext}"
extras = "{Title} - Extra.{ext}"

[thresholds]
confidence = 80

[limits]
max_files = 50000
max_path_length = 200
"#;
        let config = parse_config(toml_str).unwrap();
        assert_eq!(config.thresholds.confidence, 80);
        assert_eq!(config.templates.movie, "{Title}.{ext}");
        assert_eq!(config.limits.max_files, 50_000);
    }

    #[test]
    fn test_parse_invalid_toml() {
        let result = parse_config("not valid toml [[[");
        assert!(result.is_err());
    }

    #[test]
    fn test_serialize_roundtrip() {
        let config = default_config();
        let toml_str = serialize_config(&config).unwrap();
        let parsed = parse_config(&toml_str).unwrap();
        assert_eq!(config, parsed);
    }

    #[test]
    fn test_load_config_nonexistent_returns_default() {
        let dir = tempfile::tempdir().unwrap();
        let config = load_config(dir.path()).unwrap();
        assert_eq!(config, default_config());
    }

    #[test]
    fn test_save_and_load_config() {
        let dir = tempfile::tempdir().unwrap();
        let mut config = default_config();
        config.thresholds.confidence = 90;
        save_config(&config, dir.path()).unwrap();
        let loaded = load_config(dir.path()).unwrap();
        assert_eq!(loaded.thresholds.confidence, 90);
    }
}
