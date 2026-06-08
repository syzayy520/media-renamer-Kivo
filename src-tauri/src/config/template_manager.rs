// template_manager 模块 - 模板管理
// 职责：RenameRule 领域对象定义、模板 CRUD（按 MediaType 获取/设置模板）
// 委托：config_loader 负责持久化

use crate::config::config_loader::{default_config, load_config, save_config, AppConfig};
use crate::parse::movie_parser::MediaType;
use crate::shared::result_types::AppResult;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// 重命名规则（模板）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RenameRule {
    pub media_type: MediaType,
    pub template: String,
    pub is_default: bool,
}

/// 获取指定媒体类型的模板字符串
pub fn get_template(media_type: &MediaType, config_dir: &Path) -> AppResult<String> {
    let config = load_config(config_dir)?;
    Ok(extract_template(&config, media_type))
}

/// 设置指定媒体类型的模板字符串
pub fn set_template(media_type: &MediaType, template: &str, config_dir: &Path) -> AppResult<()> {
    let mut config = load_config(config_dir)?;
    apply_template(&mut config, media_type, template);
    save_config(&config, config_dir)
}

/// 获取所有模板列表
pub fn get_all_templates(config_dir: &Path) -> AppResult<Vec<RenameRule>> {
    let config = load_config(config_dir)?;
    let default = default_config();
    let types = [
        MediaType::Movie,
        MediaType::Series,
        MediaType::Anime,
        MediaType::Special,
        MediaType::Extras,
    ];
    let rules = types
        .iter()
        .map(|mt| {
            let template = extract_template(&config, mt);
            let default_template = extract_template(&default, mt);
            let is_default = template == default_template;
            RenameRule {
                media_type: mt.clone(),
                template,
                is_default,
            }
        })
        .collect();
    Ok(rules)
}

/// 从配置中提取指定媒体类型的模板
fn extract_template(config: &AppConfig, media_type: &MediaType) -> String {
    match media_type {
        MediaType::Movie => config.templates.movie.clone(),
        MediaType::Series => config.templates.series.clone(),
        MediaType::Anime => config.templates.anime.clone(),
        MediaType::Special | MediaType::Ova | MediaType::Ncop | MediaType::Nced => {
            config.templates.special.clone()
        }
        MediaType::Extras => config.templates.extras.clone(),
        MediaType::Unknown => "{Title}.{ext}".to_string(),
    }
}

/// 将模板应用到配置
fn apply_template(config: &mut AppConfig, media_type: &MediaType, template: &str) {
    match media_type {
        MediaType::Movie => config.templates.movie = template.to_string(),
        MediaType::Series => config.templates.series = template.to_string(),
        MediaType::Anime => config.templates.anime = template.to_string(),
        MediaType::Special | MediaType::Ova | MediaType::Ncop | MediaType::Nced => {
            config.templates.special = template.to_string()
        }
        MediaType::Extras => config.templates.extras = template.to_string(),
        MediaType::Unknown => {} // Unknown 不支持自定义模板
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn temp_dir() -> PathBuf {
        tempfile::tempdir().unwrap().keep()
    }

    #[test]
    fn test_get_default_template() {
        let dir = temp_dir();
        let tpl = get_template(&MediaType::Movie, &dir).unwrap();
        assert!(tpl.contains("{Title}"));
        assert!(tpl.contains("{Year}"));
    }

    #[test]
    fn test_set_and_get_template() {
        let dir = temp_dir();
        let custom = "{Title} [{Year}].{ext}";
        set_template(&MediaType::Movie, custom, &dir).unwrap();
        let tpl = get_template(&MediaType::Movie, &dir).unwrap();
        assert_eq!(tpl, custom);
    }

    #[test]
    fn test_get_all_templates() {
        let dir = temp_dir();
        let rules = get_all_templates(&dir).unwrap();
        assert_eq!(rules.len(), 5);
        assert!(rules.iter().all(|r| r.is_default));
    }

    #[test]
    fn test_set_template_marks_non_default() {
        let dir = temp_dir();
        set_template(&MediaType::Series, "custom-{Title}.{ext}", &dir).unwrap();
        let rules = get_all_templates(&dir).unwrap();
        let series = rules
            .iter()
            .find(|r| r.media_type == MediaType::Series)
            .unwrap();
        assert!(!series.is_default);
        let movie = rules
            .iter()
            .find(|r| r.media_type == MediaType::Movie)
            .unwrap();
        assert!(movie.is_default);
    }

    #[test]
    fn test_unknown_media_type_returns_fallback() {
        let dir = temp_dir();
        let tpl = get_template(&MediaType::Unknown, &dir).unwrap();
        assert_eq!(tpl, "{Title}.{ext}");
    }
}
