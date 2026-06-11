use super::template::RenameConflict;
use super::template::RenamePreviewItem;
use crate::shared::path_utils;

pub fn check_path_exists(item: &RenamePreviewItem) -> Option<RenameConflict> {
    if is_same_path(&item.source_path, &item.target_path) {
        return None;
    }

    None
}

fn is_same_path(left: &str, right: &str) -> bool {
    path_utils::normalize_path(left).to_lowercase() == path_utils::normalize_path(right).to_lowercase()
}
