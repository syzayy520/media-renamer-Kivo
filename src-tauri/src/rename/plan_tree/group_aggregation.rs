use super::file_item::*;
use super::folder_group::*;
use super::group_status::*;

/// 从平铺的预览项构建文件夹组树
///
/// # 分组规则
/// - 同一文件夹下的文件归为一组
/// - 没有文件夹的单独文件各自成组
/// - 同组文件类型自动识别
pub fn build_preview_tree(previews: &[FlatPreviewItem]) -> PreviewPlanTree {
    let mut groups: Vec<FolderGroup> = Vec::new();

    // 分组：按父目录分组
    let mut group_map: std::collections::HashMap<String, Vec<&FlatPreviewItem>> =
        std::collections::HashMap::new();

    for item in previews {
        group_map
            .entry(item.parent_dir.clone())
            .or_default()
            .push(item);
    }

    let mut group_id_counter: u32 = 0;

    for (parent_dir, items) in group_map {
        group_id_counter += 1;

        // 确定组媒体类型（取多数）
        let media_type = infer_group_media_type(&items);

        // 原始文件夹名
        let original_folder_name = extract_folder_name(&parent_dir);

        // 子文件列表
        let children: Vec<PreviewFileItem> = items
            .iter()
            .map(|item| PreviewFileItem {
                id: item.id.clone(),
                file_role: infer_file_role(&item.file_name, &item.extension),
                original_name: item.file_name.clone(),
                target_name: item.proposed_name.clone(),
                extension: normalize_extension(&item.extension),
                subtitle_language: item.subtitle_language.clone(),
                original_path: format!("{}\\{}", parent_dir, item.file_name),
                target_path: item.target_path.clone(),
                should_skip: false,
                needs_manual_review: item.needs_manual_review,
                metadata_source: item.metadata_source,
                safety_status: item.safety_status,
                confidence: item.confidence,
            })
            .collect();

        let child_statuses: Vec<FileSafetyStatus> =
            children.iter().map(|c| c.safety_status).collect();

        let group_status = aggregate_group_status(&child_statuses, false);

        // 目标文件夹名：从子文件推断
        let target_folder_name = infer_target_folder(&children);

        let target_path = if original_folder_name.is_empty() {
            format!("{}\\{}", parent_dir, target_folder_name)
        } else {
            parent_dir.replace(&original_folder_name, &target_folder_name)
        };

        groups.push(FolderGroup {
            id: format!("group_{:03}", group_id_counter),
            media_type,
            original_folder_name: original_folder_name.clone(),
            target_folder_name,
            original_path: parent_dir.clone(),
            target_path,
            file_count: children.len() as u32,
            tmdb_match_status: TmdbMatchStatus::NotSearched,
            folder_policy: None,
            naming_preset: Some("clean-library".to_string()),
            status: group_status,
            should_skip: false,
            needs_manual_review: items.iter().any(|i| i.needs_manual_review),
            children,
            collapsed: false,
        });
    }

    let total_groups = groups.len() as u32;
    let total_files = previews.len() as u32;

    PreviewPlanTree {
        groups,
        total_files,
        total_groups,
        skipped_groups: 0,
        blocked_groups: 0,
    }
}

/// 用于从已有系统传入的预览项
#[derive(Debug, Clone)]
pub struct FlatPreviewItem {
    pub id: String,
    pub file_name: String,
    pub proposed_name: String,
    pub extension: String,
    pub parent_dir: String,
    pub target_path: String,
    pub subtitle_language: Option<String>,
    pub needs_manual_review: bool,
    pub metadata_source: MetadataSource,
    pub safety_status: FileSafetyStatus,
    pub confidence: f32,
}

/// 推断组媒体类型
fn infer_group_media_type(items: &[&FlatPreviewItem]) -> GroupMediaType {
    // 默认：从扩展名推断
    let has_tv_pattern = items.iter().any(|i| {
        let name = i.file_name.to_lowercase();
        name.contains("s01")
            || name.contains("s02")
            || name.contains("s03")
            || name.contains("e01")
            || name.contains("e02")
            || name.contains("1x01")
            || name.contains("ep01")
            || name.contains("season")
    });

    if has_tv_pattern {
        GroupMediaType::Tv
    } else {
        GroupMediaType::Movie
    }
}

/// 提取文件夹名（路径最后一段）
fn extract_folder_name(path: &str) -> String {
    path.rsplit('\\').next().unwrap_or(path).to_string()
}

/// 推断目标文件夹名（从子文件目标名反推）
fn infer_target_folder(children: &[PreviewFileItem]) -> String {
    // 从第一个主视频文件的目标名中去掉扩展名，作为目标文件夹前缀
    let video = children.iter().find(|c| c.file_role == FileRole::MainVideo);
    if let Some(v) = video {
        let name = &v.target_name;
        // 去掉扩展名部分（最后一段以 .开头）
        if let Some(dot_idx) = name.rfind('.') {
            let without_ext = &name[..dot_idx];
            return without_ext.to_string();
        }
        return name.clone();
    }
    // 回退：使用第一个子文件名
    children
        .first()
        .map(|c| {
            let name = &c.target_name;
            if let Some(dot_idx) = name.rfind('.') {
                name[..dot_idx].to_string()
            } else {
                name.clone()
            }
        })
        .unwrap_or_default()
}

/// 推断文件角色
fn infer_file_role(file_name: &str, extension: &str) -> FileRole {
    let lower = file_name.to_lowercase();
    let ext = extension.to_lowercase();

    // 字幕文件
    if matches!(ext.as_str(), "srt" | "ass" | "ssa" | "sub" | "idx" | "vtt") {
        return FileRole::Subtitle;
    }

    // NFO 文件
    if ext == "nfo" || ext == "xml" {
        return FileRole::Nfo;
    }

    // 图片文件
    if matches!(
        ext.as_str(),
        "jpg" | "jpeg" | "png" | "gif" | "webp" | "bmp"
    ) && (lower.contains("poster")
        || lower.contains("folder")
        || lower.contains("cover")
        || lower.contains("fanart")
        || lower.contains("backdrop")
        || lower.contains("banner"))
    {
        return FileRole::Image;
    }

    // 视频文件
    if matches!(
        ext.as_str(),
        "mkv"
            | "mp4"
            | "avi"
            | "m2ts"
            | "ts"
            | "mov"
            | "wmv"
            | "flv"
            | "webm"
            | "rmvb"
            | "iso"
            | "bdmv"
    ) {
        return FileRole::MainVideo;
    }

    // 可能的字幕文件（基于文件名）
    if lower.contains(".zh.")
        || lower.contains(".chs.")
        || lower.contains(".cht.")
        || lower.contains(".en.")
        || lower.contains(".jp.")
    {
        return FileRole::Subtitle;
    }

    FileRole::Extra
}

/// 规范化扩展名（统一去掉前导点）
fn normalize_extension(raw: &str) -> String {
    let trimmed = raw.trim().trim_start_matches('.');
    if trimmed.is_empty() {
        String::new()
    } else {
        format!(".{}", trimmed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_preview(
        id: &str,
        name: &str,
        proposed: &str,
        ext: &str,
        parent: &str,
    ) -> FlatPreviewItem {
        FlatPreviewItem {
            id: id.to_string(),
            file_name: name.to_string(),
            proposed_name: proposed.to_string(),
            extension: ext.to_string(),
            parent_dir: parent.to_string(),
            target_path: format!("{}\\{}", parent, proposed),
            subtitle_language: None,
            needs_manual_review: false,
            metadata_source: MetadataSource::Parsed,
            safety_status: FileSafetyStatus::Ready,
            confidence: 0.9,
        }
    }

    #[test]
    fn test_build_movie_folder_group() {
        let items = vec![
            make_preview(
                "1",
                "First.Blood.1982.mkv",
                "第一滴血 (1982).mkv",
                "mkv",
                "D:\\Movies\\Rambo.First.Blood",
            ),
            make_preview(
                "2",
                "First.Blood.1982.zh.srt",
                "第一滴血 (1982).zh.srt",
                "srt",
                "D:\\Movies\\Rambo.First.Blood",
            ),
        ];

        let tree = build_preview_tree(&items);
        assert_eq!(tree.groups.len(), 1);
        let group = &tree.groups[0];
        assert_eq!(group.original_folder_name, "Rambo.First.Blood");
        assert_eq!(group.media_type, GroupMediaType::Movie);
        assert_eq!(group.children.len(), 2);
        assert_eq!(group.children[0].file_role, FileRole::MainVideo);
        assert_eq!(group.children[1].file_role, FileRole::Subtitle);
    }

    #[test]
    fn test_build_tv_folder_group() {
        let items = vec![
            make_preview(
                "1",
                "Show.S01E01.mkv",
                "最后生还者 S01E01.mkv",
                "mkv",
                "D:\\TV\\The.Last.of.Us.S01",
            ),
            make_preview(
                "2",
                "Show.S01E02.mkv",
                "最后生还者 S01E02.mkv",
                "mkv",
                "D:\\TV\\The.Last.of.Us.S01",
            ),
        ];

        let tree = build_preview_tree(&items);
        assert_eq!(tree.groups.len(), 1);
        assert_eq!(tree.groups[0].media_type, GroupMediaType::Tv);
    }

    #[test]
    fn test_infer_file_role() {
        assert_eq!(infer_file_role("movie.mkv", "mkv"), FileRole::MainVideo);
        assert_eq!(infer_file_role("subtitle.srt", "srt"), FileRole::Subtitle);
        assert_eq!(infer_file_role("movie.nfo", "nfo"), FileRole::Nfo);
        assert_eq!(infer_file_role("poster.jpg", "jpg"), FileRole::Image);
        assert_eq!(infer_file_role("info.txt", "txt"), FileRole::Extra);
    }

    #[test]
    fn test_normalize_extension() {
        assert_eq!(normalize_extension("mkv"), ".mkv");
        assert_eq!(normalize_extension(".mkv"), ".mkv");
        assert_eq!(normalize_extension(""), "");
    }

    #[test]
    fn test_infer_target_folder_from_main_video() {
        let children = vec![PreviewFileItem {
            id: "1".into(),
            file_role: FileRole::MainVideo,
            original_name: "movie.mkv".into(),
            target_name: "第一滴血 (1982).mkv".into(),
            extension: ".mkv".into(),
            subtitle_language: None,
            original_path: String::new(),
            target_path: String::new(),
            should_skip: false,
            needs_manual_review: false,
            metadata_source: MetadataSource::Parsed,
            safety_status: FileSafetyStatus::Ready,
            confidence: 1.0,
        }];
        assert_eq!(infer_target_folder(&children), "第一滴血 (1982)");
    }
}
