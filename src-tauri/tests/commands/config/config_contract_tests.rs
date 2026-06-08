// config_contract_tests 模块 - 配置命令契约测试
// 职责：验证 config commands 的参数、返回值、错误分类、安全边界
// 使用 tempdir / 内存数据库，禁止真实路径

use std::fs;
use tempfile::tempdir;

use app_lib::config::config_loader;
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
