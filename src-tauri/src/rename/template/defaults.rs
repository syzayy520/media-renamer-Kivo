// rename/template/defaults.rs
// 职责：提供各媒体类型的默认重命名模板

use crate::parse::movie_parser::MediaType;

/// 获取默认模板
pub fn get_default_template(media_type: &MediaType) -> &'static str {
    match media_type {
        MediaType::Movie => "{Title} ({Year}){ext}",
        MediaType::Series => "{Title} - S{Season:02}E{Episode:02} - {EpisodeTitle}{ext}",
        MediaType::Anime => "{Title} - S{Season:02}E{Episode:02}{ext}",
        MediaType::Special | MediaType::Ova | MediaType::Ncop | MediaType::Nced => {
            "{Title} - S00E{Episode:02} - {SpecialType}{ext}"
        }
        MediaType::Extras => "{Title} - Extra - {ExtraType}{ExtraNumber}{ext}",
        MediaType::Unknown => "{Title}{ext}",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_default_template_movie() {
        let template = get_default_template(&MediaType::Movie);
        assert_eq!(template, "{Title} ({Year}){ext}");
    }

    #[test]
    fn test_get_default_template_series() {
        let template = get_default_template(&MediaType::Series);
        assert!(template.contains("{Title}"));
        assert!(template.contains("{Season:02}"));
        assert!(template.contains("{Episode:02}"));
    }

    #[test]
    fn test_get_default_template_anime() {
        let template = get_default_template(&MediaType::Anime);
        assert_eq!(template, "{Title} - S{Season:02}E{Episode:02}{ext}");
    }

    #[test]
    fn test_get_default_template_extras() {
        let template = get_default_template(&MediaType::Extras);
        assert!(template.contains("{ExtraType}"));
        assert!(template.contains("{ExtraNumber}"));
    }

    #[test]
    fn test_default_templates_dont_have_literal_dot_ext() {
        // 验证所有默认模板不使用字面量 ".{ext}" 以避免尾随点
        let templates = [
            ("Movie", get_default_template(&MediaType::Movie)),
            ("Series", get_default_template(&MediaType::Series)),
            ("Anime", get_default_template(&MediaType::Anime)),
            ("Special", get_default_template(&MediaType::Special)),
            ("Extras", get_default_template(&MediaType::Extras)),
            ("Unknown", get_default_template(&MediaType::Unknown)),
        ];
        for (name, template) in &templates {
            assert!(
                !template.contains(".{ext}"),
                "Template '{}' contains '.{{ext}}' which causes trailing dot when extension is empty",
                name
            );
            assert!(
                !template.contains(".{Ext}"),
                "Template '{}' contains '.{{Ext}}' which causes trailing dot when extension is empty",
                name
            );
        }
    }
}
