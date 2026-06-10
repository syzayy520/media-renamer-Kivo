// rename/candidate_apply/target_name_builder 模块 - 目标名称构建
// 职责：根据媒体类型和候选信息构建目标文件名

use crate::rename::template::RenamePreviewItem;
use crate::shared::path_utils;

/// 构建电影目标文件名
///
/// 格式: "Title (Year).ext"
///
/// # Arguments
/// * `title` - 电影标题
/// * `year` - 年份（可选）
/// * `extension` - 文件扩展名
pub fn build_movie_target_name(
    title: &str,
    year: Option<u16>,
    extension: &str,
) -> Result<String, String> {
    if title.is_empty() {
        return Err("电影标题不能为空".to_string());
    }

    let sanitized_title = path_utils::sanitize_filename(title);
    let year_str = year.map(|y| format!(" ({})", y)).unwrap_or_default();
    let name = format!("{}{}{}", sanitized_title, year_str, extension);

    // 验证文件名长度
    if name.len() > 255 {
        return Err("生成的文件名过长".to_string());
    }

    Ok(name)
}

/// 构建剧集目标文件名
///
/// 格式: "Title - S01E05 - Episode Title.ext"
///
/// # Arguments
/// * `title` - 剧集标题
/// * `season` - 季号
/// * `episode` - 集号
/// * `episode_title` - 集标题（可选）
/// * `extension` - 文件扩展名
pub fn build_tv_target_name(
    title: &str,
    season: Option<u16>,
    episode: Option<u16>,
    episode_title: Option<&str>,
    extension: &str,
) -> Result<String, String> {
    if title.is_empty() {
        return Err("剧集标题不能为空".to_string());
    }

    let sanitized_title = path_utils::sanitize_filename(title);
    let season_str = season
        .map(|s| format!("S{:02}", s))
        .unwrap_or_else(|| "S00".to_string());
    let episode_str = episode
        .map(|e| format!("E{:02}", e))
        .unwrap_or_else(|| "E00".to_string());
    let episode_title_str = episode_title
        .map(|t| format!(" - {}", path_utils::sanitize_filename(t)))
        .unwrap_or_default();

    let name = format!(
        "{} - {}{}{}{}",
        sanitized_title, season_str, episode_str, episode_title_str, extension
    );

    // 验证文件名长度
    if name.len() > 255 {
        return Err("生成的文件名过长".to_string());
    }

    Ok(name)
}

/// 更新预览项的目标名称
///
/// # Arguments
/// * `item` - 原始预览项
/// * `new_proposed_name` - 新的提议名称
pub fn update_preview_item_target(
    item: &RenamePreviewItem,
    new_proposed_name: &str,
) -> RenamePreviewItem {
    let parent_dir = std::path::Path::new(&item.source_path)
        .parent()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_default();

    let new_target_path = format!("{}\\{}", parent_dir, new_proposed_name);

    RenamePreviewItem {
        proposed_name: new_proposed_name.to_string(),
        target_path: new_target_path,
        ..item.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_movie_target_name() {
        let result = build_movie_target_name("Inception", Some(2010u16), ".mkv");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Inception (2010).mkv");
    }

    #[test]
    fn test_build_movie_target_name_no_year() {
        let result = build_movie_target_name("Inception", None, ".mkv");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Inception.mkv");
    }

    #[test]
    fn test_build_movie_target_name_empty_title() {
        let result = build_movie_target_name("", Some(2010u16), ".mkv");
        assert!(result.is_err());
    }

    #[test]
    fn test_build_tv_target_name() {
        let result = build_tv_target_name(
            "Breaking Bad",
            Some(1u16),
            Some(5u16),
            Some("Gray Matter"),
            ".mkv",
        );
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Breaking Bad - S01E05 - Gray Matter.mkv");
    }

    #[test]
    fn test_build_tv_target_name_no_episode_title() {
        let result = build_tv_target_name("Breaking Bad", Some(1u16), Some(5u16), None, ".mkv");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Breaking Bad - S01E05.mkv");
    }

    #[test]
    fn test_build_tv_target_name_empty_title() {
        let result = build_tv_target_name("", Some(1u16), Some(5u16), None, ".mkv");
        assert!(result.is_err());
    }
}
