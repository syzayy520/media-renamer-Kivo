// Unicode 文件名夹具
// 职责：提供各种 Unicode 文件名测试数据

use crate::test_support_preview_item_builder::PreviewItemBuilder;
use app_lib::rename::template::RenamePreviewItem;

/// 获取 Unicode 文件名测试对（源路径, 目标路径）
#[allow(dead_code)]
pub fn unicode_filename_pairs() -> Vec<(&'static str, &'static str)> {
    vec![
        // 中文
        ("C:\\media\\电影.mkv", "C:\\media\\电影 (2020).mkv"),
        ("C:\\media\\测试文件.mkv", "C:\\media\\重命名后.mkv"),
        // 日文
        ("C:\\media\\アニメ.mkv", "C:\\media\\アニメ (2020).mkv"),
        // 韩文
        ("C:\\media\\한국.mkv", "C:\\media\\한국 (2020).mkv"),
        // 阿拉伯文
        ("C:\\media\\فيلم.mkv", "C:\\media\\فيلم (2020).mkv"),
        // 西里尔文
        ("C:\\media\\фильм.mkv", "C:\\media\\фильм (2020).mkv"),
        // 混合中英文
        ("C:\\media\\Movie 电影.mkv", "C:\\media\\Movie 电影 (2020).mkv"),
        // 特殊符号
        ("C:\\media\\file (1).mkv", "C:\\media\\File One (2020).mkv"),
    ]
}

/// 生成 Unicode 文件名预览项
#[allow(dead_code)]
pub fn generate_unicode_items() -> Vec<RenamePreviewItem> {
    unicode_filename_pairs()
        .into_iter()
        .enumerate()
        .map(|(i, (source, target))| {
            PreviewItemBuilder::new(i)
                .source_path(source)
                .target_path(target)
                .build()
        })
        .collect()
}

/// 生成大量 Unicode 文件名项（每项唯一目标路径）
#[allow(dead_code)]
pub fn generate_unicode_items_bulk(count: usize) -> Vec<RenamePreviewItem> {
    let pairs = unicode_filename_pairs();
    (0..count)
        .map(|i| {
            let (source, _template_target) = pairs[i % pairs.len()];
            // Append index to make each target path unique
            let base_target = format!("C:\\media\\unique_{}", i);
            let ext = std::path::Path::new(source)
                .extension()
                .map(|e| format!(".{}", e.to_string_lossy()))
                .unwrap_or_else(|| ".mkv".to_string());
            PreviewItemBuilder::new(i)
                .source_path(source)
                .target_path(&format!("{}{}", base_target, ext))
                .build()
        })
        .collect()
}
