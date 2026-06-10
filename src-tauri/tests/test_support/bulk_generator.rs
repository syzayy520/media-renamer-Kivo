// 批量生成器
// 职责：生成大量测试预览项

use crate::test_support_preview_item_builder::PreviewItemBuilder;
use app_lib::parse::movie_parser::MediaType;
use app_lib::rename::template::RenamePreviewItem;

/// 生成 N 个唯一的预览项（不同源路径、不同目标路径）
#[allow(dead_code)]
pub fn generate_unique_items(count: usize) -> Vec<RenamePreviewItem> {
    (0..count)
        .map(|i| PreviewItemBuilder::new(i).build())
        .collect()
}

/// 生成 N 个安全项（高置信度、无冲突、无跳过）
#[allow(dead_code)]
pub fn generate_safe_items(count: usize) -> Vec<RenamePreviewItem> {
    (0..count)
        .map(|i| {
            PreviewItemBuilder::new(i)
                .confidence(90)
                .needs_manual_review(false)
                .should_skip(false)
                .build()
        })
        .collect()
}

/// 生成混合置信度项（高/低交替）
#[allow(dead_code)]
pub fn generate_mixed_confidence_items(count: usize) -> Vec<RenamePreviewItem> {
    (0..count)
        .map(|i| {
            let conf = if i % 2 == 0 { 90 } else { 40 };
            PreviewItemBuilder::new(i).confidence(conf).build()
        })
        .collect()
}

/// 生成混合媒体类型项（循环 Movie/Series/Anime/Special）
#[allow(dead_code)]
pub fn generate_mixed_media_items(count: usize) -> Vec<RenamePreviewItem> {
    let types = [
        MediaType::Movie,
        MediaType::Series,
        MediaType::Anime,
        MediaType::Special,
    ];
    (0..count)
        .map(|i| {
            PreviewItemBuilder::new(i)
                .media_type(types[i % types.len()].clone())
                .build()
        })
        .collect()
}

/// 生成具有相同目标路径的冲突项
#[allow(dead_code)]
pub fn generate_conflict_items(count: usize) -> Vec<RenamePreviewItem> {
    (0..count)
        .map(|i| {
            PreviewItemBuilder::new(i)
                .target_path("C:\\media\\same_target.mkv")
                .build()
        })
        .collect()
}
