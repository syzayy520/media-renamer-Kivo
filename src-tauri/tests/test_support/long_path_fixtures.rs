// 长路径夹具
// 职责：提供长路径测试数据

use crate::test_support_preview_item_builder::PreviewItemBuilder;
use app_lib::rename::template::RenamePreviewItem;

/// 生成刚好超过 260 字符的路径 (261 chars)
#[allow(dead_code)]
pub fn path_just_over_limit() -> String {
    // "C:\media\" = 9, ".mkv" = 4 → need 248 'a' chars for 261 total
    let padding = "a".repeat(248);
    format!("C:\\media\\{}.mkv", padding)
}

/// 生成深度嵌套路径（260+ 字符）
#[allow(dead_code)]
pub fn deep_nested_path() -> String {
    let mut path = String::from("C:\\");
    // "dir_00\" = 8 chars each, need enough to exceed 260
    for i in 0..35 {
        path.push_str(&format!("dir_{:02}\\", i));
    }
    path.push_str("file.mkv");
    path
}

/// 生成长文件名路径 (260+ chars)
#[allow(dead_code)]
pub fn long_filename_path() -> String {
    let long_name = "Very_Long_Movie_Title_With_Many_Words_And_Details_More".repeat(6);
    format!("C:\\media\\{}.mkv", long_name)
}

/// 生成长路径预览项
#[allow(dead_code)]
pub fn generate_long_path_items() -> Vec<RenamePreviewItem> {
    let paths = vec![
        path_just_over_limit(),
        deep_nested_path(),
        long_filename_path(),
    ];
    paths.into_iter()
        .enumerate()
        .map(|(i, target)| {
            PreviewItemBuilder::new(i)
                .target_path(&target)
                .build()
        })
        .collect()
}

/// 生成大量长路径项
#[allow(dead_code)]
pub fn generate_long_path_items_bulk(count: usize) -> Vec<RenamePreviewItem> {
    (0..count)
        .map(|i| {
            let padding = "x".repeat(250);
            let target = format!("C:\\media\\{}_{}.mkv", i, padding);
            PreviewItemBuilder::new(i)
                .target_path(&target)
                .build()
        })
        .collect()
}
