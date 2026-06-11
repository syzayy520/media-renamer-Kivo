use crate::rename::template::RenamePreviewItem;

pub fn original_release_name(item: &RenamePreviewItem) -> String {
    item.parsed_info.media_item.file_name.clone()
}

pub fn original_name_without_ext(item: &RenamePreviewItem) -> String {
    item.parsed_info.media_item.file_name.clone()
}
