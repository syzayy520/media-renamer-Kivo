// 类型分类器模块
// 职责：根据文件名特征判定媒体类型，调用各 parser 获取最佳解析结果
// 优先级：special > anime explicit pattern > series > movie > unknown
// 禁止：parse_helper / regex_utils / media_utils 等万能垃圾桶

use super::anime_parser;
use super::movie_parser::{MediaType, ParsedMediaInfo, SpecialType};
use super::series_parser;
use super::special_parser;

/// 快速分类（仅判断类型，不解析完整信息）
/// 优先级：extras > special > anime > series > movie > unknown
pub fn classify(filename: &str) -> MediaType {
    let lower = filename.to_lowercase();

    // 1. Extras（必须在 NCOP/NCED 之前，避免 "Extra - NCOP01" 误判）
    if lower.contains("extra") {
        return MediaType::Extras;
    }

    // 2. NCOP/NCED
    if lower.contains("ncop") {
        return MediaType::Ncop;
    }
    if lower.contains("nced") {
        return MediaType::Nced;
    }

    // 3. OVA/OAD
    if lower.contains("ova") || lower.contains("oad") {
        return MediaType::Ova;
    }

    // 4. SP / 特别篇
    if is_sp_keyword(&lower) {
        return MediaType::Special;
    }

    // 5. S00E01 特别篇格式
    if regex::Regex::new(r"(?i)S00E\d{1,3}")
        .ok()
        .and_then(|re| re.find(filename))
        .is_some()
    {
        return MediaType::Special;
    }

    // 6. 动漫特征：[Group] 标记
    if filename.starts_with('[') && filename.contains("] ") {
        return MediaType::Anime;
    }

    // 7. 剧集特征：S01E01 格式
    if regex::Regex::new(r"(?i)S\d{1,2}E\d{1,3}")
        .ok()
        .and_then(|re| re.find(filename))
        .is_some()
    {
        return MediaType::Series;
    }

    // 8. 电影特征：年份
    if regex::Regex::new(r"\(?(19|20)\d{2}\)?")
        .ok()
        .and_then(|re| re.find(filename))
        .is_some()
    {
        return MediaType::Movie;
    }

    MediaType::Unknown
}

/// 判定特别篇子类型
pub fn classify_special(filename: &str) -> SpecialType {
    let lower = filename.to_lowercase();

    if lower.contains("ova") {
        return SpecialType::Ova;
    }
    if lower.contains("oad") {
        return SpecialType::Special; // OAD 归类为 Special
    }
    if lower.contains("ncop") {
        return SpecialType::Ncop;
    }
    if lower.contains("nced") {
        return SpecialType::Nced;
    }
    if lower.contains("extra") {
        return SpecialType::Extra;
    }
    if is_sp_keyword(&lower) {
        return SpecialType::Sp;
    }

    SpecialType::Special
}

/// 检测 SP 关键字（避免误匹配 "space"、"special" 等词）
fn is_sp_keyword(lower: &str) -> bool {
    lower.contains(" sp ")
        || lower.contains("-sp-")
        || lower.contains("[sp]")
        || lower.contains(".sp.")
        || lower.contains("-sp")
        || lower.contains("sp-")
        || lower.contains("- sp")
        || lower.contains(".sp")
}

/// 完整分类 + 解析
/// 调用各 parser，返回置信度最高的解析结果
/// 优先级：special > anime > series > movie
pub fn classify_and_parse(filename: &str) -> Option<ParsedMediaInfo> {
    // 1. 尝试特别篇（最高优先级）
    if let Some(info) = special_parser::parse_special(filename) {
        return Some(info);
    }

    // 2. 尝试动漫（明确的 [Group] 模式）
    if let Some(info) = anime_parser::parse_anime(filename) {
        return Some(info);
    }

    // 3. 尝试剧集
    if let Some(info) = series_parser::parse_series(filename) {
        return Some(info);
    }

    // 4. 尝试电影
    if let Some(info) = super::movie_parser::parse_movie(filename) {
        return Some(info);
    }

    // 5. 未识别
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    // === classify 快速分类测试 ===

    #[test]
    fn test_classify_anime_with_group() {
        assert_eq!(classify("[SubGroup] Title - S01E01"), MediaType::Anime);
    }

    #[test]
    fn test_classify_series() {
        assert_eq!(classify("Title - S01E01 - Episode"), MediaType::Series);
    }

    #[test]
    fn test_classify_movie() {
        assert_eq!(classify("The Matrix (1999)"), MediaType::Movie);
    }

    #[test]
    fn test_classify_ova() {
        assert_eq!(classify("Title - OVA01"), MediaType::Ova);
    }

    #[test]
    fn test_classify_ncop_nced() {
        // 修复：NCED 不应返回 Ncop
        assert_eq!(classify("Title - NCOP01"), MediaType::Ncop);
        assert_eq!(classify("Title - NCED01"), MediaType::Nced);
    }

    #[test]
    fn test_classify_sp() {
        assert_eq!(classify("Title - SP01"), MediaType::Special);
        assert_eq!(classify("Title.SP02"), MediaType::Special);
    }

    #[test]
    fn test_classify_s00_special() {
        assert_eq!(classify("Title - S00E01 - Prologue"), MediaType::Special);
    }

    #[test]
    fn test_classify_extras() {
        assert_eq!(classify("Title - Extra - NCOP01"), MediaType::Extras);
    }

    #[test]
    fn test_classify_unknown() {
        assert_eq!(classify("random_file"), MediaType::Unknown);
    }

    // === classify_special 子类型测试 ===

    #[test]
    fn test_classify_special_ova() {
        assert_eq!(classify_special("Title - OVA01"), SpecialType::Ova);
    }

    #[test]
    fn test_classify_special_ncop() {
        assert_eq!(classify_special("Title - NCOP01"), SpecialType::Ncop);
    }

    #[test]
    fn test_classify_special_nced() {
        assert_eq!(classify_special("Title - NCED01"), SpecialType::Nced);
    }

    // === classify_and_parse 完整解析测试 ===

    #[test]
    fn test_classify_and_parse_series() {
        let result = classify_and_parse("Breaking Bad - S01E01 - Pilot [1080p]");
        assert!(result.is_some());
        let info = result.unwrap();
        assert_eq!(info.media_type, MediaType::Series);
    }

    #[test]
    fn test_classify_and_parse_anime() {
        let result = classify_and_parse("[SubGroup] Title - 01 [1080p]");
        assert!(result.is_some());
        let info = result.unwrap();
        assert_eq!(info.media_type, MediaType::Anime);
    }

    #[test]
    fn test_classify_and_parse_special() {
        let result = classify_and_parse("Naruto - S00E01 - Prologue");
        assert!(result.is_some());
        let info = result.unwrap();
        assert_eq!(info.media_type, MediaType::Special);
    }

    #[test]
    fn test_classify_and_parse_movie() {
        let result = classify_and_parse("The Matrix (1999) [1080p] [BluRay]");
        assert!(result.is_some());
        let info = result.unwrap();
        assert_eq!(info.media_type, MediaType::Movie);
    }

    #[test]
    fn test_classify_and_parse_unknown() {
        let result = classify_and_parse("random_file.mkv");
        assert!(result.is_none());
    }
}
