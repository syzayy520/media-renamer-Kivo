use crate::media_info::contracts::NormalizedMediaInfo;
use crate::media_info::probe::filename_media_info_parser::parse_filename_media_info;
use crate::rename::template::RenamePreviewItem;

pub fn media_info_for_item(item: &RenamePreviewItem) -> NormalizedMediaInfo {
    parse_filename_media_info(&item.parsed_info.media_item.file_name)
}

pub fn first_value(primary: Option<String>, fallback: Option<String>) -> Option<String> {
    primary.or(fallback)
}
