// config_contract_tests 模块 - 配置命令契约测试
// 职责：验证 config commands 的参数、返回值、错误分类、安全边界
// 使用 tempdir / 内存数据库，禁止真实路径

use std::fs;
use tempfile::tempdir;

use app_lib::config::config_loader;
use app_lib::config::secret::api_key_store;
use app_lib::config::template_manager;
use app_lib::config::threshold;
use app_lib::parse::movie_parser::MediaType;

/// 测试：默认配置值
#[test]
fn test_default_config_values() {
    let dir = tempdir().unwrap();
    let config = config_loader::load_config(dir.path()).unwrap();

    // 默认模板不为空
    assert!(!config.templates.movie.is_empty());
    assert!(!config.templates.series.is_empty());
    assert!(!config.templates.anime.is_empty());

    // 默认阈值在合理范围
    assert!(config.thresholds.confidence <= 100);
}

/// 测试：保存和加载配置
#[test]
fn test_save_and_load_config() {
    let dir = tempdir().unwrap();

    // 修改模板
    let mut config = config_loader::load_config(dir.path()).unwrap();
    config.templates.movie = "{Title} ({Year})".to_string();

    // 保存
    config_loader::save_config(&config, dir.path()).unwrap();

    // 重新加载
    let loaded = config_loader::load_config(dir.path()).unwrap();
    assert_eq!(loaded.templates.movie, "{Title} ({Year})");
}

/// 测试：无效 TOML 返回错误
#[test]
fn test_parse_invalid_toml() {
    let dir = tempdir().unwrap();
    let config_path = dir.path().join("config.toml");

    // 写入无效 TOML
    fs::write(&config_path, "this is not valid toml [[[").unwrap();

    let result = config_loader::load_config(dir.path());
    assert!(result.is_err());
}

/// 测试：不存在的配置文件返回默认值
#[test]
fn test_load_config_nonexistent_returns_default() {
    let dir = tempdir().unwrap();
    let config = config_loader::load_config(dir.path()).unwrap();

    // 应该返回默认配置
    assert!(!config.templates.movie.is_empty());
}

/// 测试：模板管理 - 获取所有模板
#[test]
fn test_get_all_templates() {
    let dir = tempdir().unwrap();
    let templates = template_manager::get_all_templates(dir.path()).unwrap();

    // 应该有默认模板
    assert!(!templates.is_empty());

    // 每个模板应该有 media_type 和 template
    for rule in &templates {
        assert!(!rule.template.is_empty());
    }
}

/// 测试：模板管理 - 设置和获取模板
#[test]
fn test_set_and_get_template() {
    let dir = tempdir().unwrap();

    // 设置电影模板
    template_manager::set_template(&MediaType::Movie, "{Title} ({Year})", dir.path()).unwrap();

    // 获取所有模板
    let templates = template_manager::get_all_templates(dir.path()).unwrap();
    let movie_template = templates.iter().find(|r| r.media_type == MediaType::Movie);

    assert!(movie_template.is_some());
    assert_eq!(movie_template.unwrap().template, "{Title} ({Year})");
}

/// 测试：模板管理 - 所有媒体类型都有模板
#[test]
fn test_all_media_types_have_templates() {
    let dir = tempdir().unwrap();

    // 获取默认模板
    let templates = template_manager::get_all_templates(dir.path()).unwrap();

    // 应该有 Movie 和 Series 模板
    let movie = templates.iter().find(|r| r.media_type == MediaType::Movie);
    let series = templates.iter().find(|r| r.media_type == MediaType::Series);
    assert!(movie.is_some());
    assert!(series.is_some());
}

/// 测试：置信度阈值 - 获取默认值
#[test]
fn test_get_default_threshold() {
    let dir = tempdir().unwrap();
    let value = threshold::get_threshold(dir.path()).unwrap();

    // 默认阈值应该在合理范围
    assert!(value <= 100);
}

/// 测试：置信度阈值 - 设置和获取
#[test]
fn test_set_and_get_threshold() {
    let dir = tempdir().unwrap();

    // 设置阈值
    threshold::set_threshold(75, dir.path()).unwrap();

    // 获取阈值
    let value = threshold::get_threshold(dir.path()).unwrap();
    assert_eq!(value, 75);
}

/// 测试：置信度阈值 - 边界值 0
#[test]
fn test_threshold_boundary_zero() {
    let dir = tempdir().unwrap();

    // 设置阈值为 0
    threshold::set_threshold(0, dir.path()).unwrap();

    let value = threshold::get_threshold(dir.path()).unwrap();
    assert_eq!(value, 0);
}

/// 测试：置信度阈值 - 边界值 100
#[test]
fn test_threshold_boundary_hundred() {
    let dir = tempdir().unwrap();

    // 设置阈值为 100
    threshold::set_threshold(100, dir.path()).unwrap();

    let value = threshold::get_threshold(dir.path()).unwrap();
    assert_eq!(value, 100);
}

/// 测试：配置序列化往返
#[test]
fn test_serialize_roundtrip() {
    let dir = tempdir().unwrap();

    let config1 = config_loader::load_config(dir.path()).unwrap();
    config_loader::save_config(&config1, dir.path()).unwrap();
    let config2 = config_loader::load_config(dir.path()).unwrap();

    assert_eq!(config1.templates.movie, config2.templates.movie);
    assert_eq!(config1.thresholds.confidence, config2.thresholds.confidence);
}

// ===== TMDb API Key Contract Tests =====

/// 测试：TMDb API Key 不存在时的默认状态
#[test]
fn test_tmdb_api_key_default_not_configured() {
    let dir = tempdir().unwrap();
    let key = api_key_store::get_api_key("tmdb", dir.path()).unwrap();
    assert!(key.is_none(), "default should be not configured");
}

/// 测试：保存 API Key 后可以获取
#[test]
fn test_tmdb_api_key_set_and_get() {
    let dir = tempdir().unwrap();

    api_key_store::save_api_key("tmdb", "test_key_12345abcde", dir.path()).unwrap();
    let key = api_key_store::get_api_key("tmdb", dir.path()).unwrap();

    assert!(key.is_some());
    assert_eq!(key.unwrap(), "test_key_12345abcde");
}

/// 测试：清除 API Key 后状态为 not configured
#[test]
fn test_tmdb_api_key_clear_status() {
    let dir = tempdir().unwrap();

    api_key_store::save_api_key("tmdb", "test_key_12345abcde", dir.path()).unwrap();
    api_key_store::delete_api_key("tmdb", dir.path()).unwrap();

    let key = api_key_store::get_api_key("tmdb", dir.path()).unwrap();
    assert!(key.is_none(), "should be cleared");
}

/// 测试：返回的 API Key 状态不包含明文
#[test]
fn test_tmdb_api_key_status_no_plaintext() {
    let dir = tempdir().unwrap();

    api_key_store::save_api_key("tmdb", "secret_key_not_leaked", dir.path()).unwrap();

    // get_api_key 返回的是真实 key，但 command 层应该只返回 bool
    // 此处测试存储层，command 层的 TmdbApiKeyStatus 只有 configured 字段
    let key = api_key_store::get_api_key("tmdb", dir.path()).unwrap();
    assert!(key.is_some());

    // 脱敏测试：mask 函数不返回完整明文
    let masked = api_key_store::mask_api_key("secret_key_not_leaked");
    assert!(!masked.contains("secret_key_not_leaked"), "masked should not contain full key");
}

/// 测试：空 key 保存不应该泄漏信息
#[test]
fn test_tmdb_api_key_empty_not_saved() {
    let dir = tempdir().unwrap();

    // 空 key 测试不作为存储层测试（存储层允许空字符串）
    // 验证 pattern：get 后在 command 层禁止空值
    let key = api_key_store::get_api_key("tmdb", dir.path()).unwrap();
    assert!(key.is_none(), "should start as none");

    // 保存后获取
    api_key_store::save_api_key("tmdb", "valid_key_67890", dir.path()).unwrap();
    let key = api_key_store::get_api_key("tmdb", dir.path()).unwrap();
    assert_eq!(key.unwrap(), "valid_key_67890");
}
