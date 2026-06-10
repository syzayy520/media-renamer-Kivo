// src-tauri/src/scrape/local_metadata/nfo_document_builder.rs
// 职责：把 TMDb 候选构造成本地 NFO XML 文档

use crate::tmdb_search_contract::{TmdbCandidate, TmdbSearchMediaType};

pub fn build_nfo_document(candidate: &TmdbCandidate) -> String {
    let root = match candidate.media_type {
        TmdbSearchMediaType::Movie => "movie",
        TmdbSearchMediaType::Tv => "tvshow",
    };

    let mut xml = String::new();
    xml.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\n");
    xml.push_str(&format!("<{}>\n", root));
    push_tag(&mut xml, "title", &candidate.title);

    if let Some(original_title) = &candidate.original_title {
        push_tag(&mut xml, "originaltitle", original_title);
    }

    if let Some(year) = candidate.release_year {
        push_tag(&mut xml, "year", &year.to_string());
    }

    if let Some(overview) = &candidate.overview {
        push_tag(&mut xml, "plot", overview);
    }

    push_tag(&mut xml, "tmdbid", &candidate.tmdb_id.to_string());
    push_tag(&mut xml, "uniqueid", &candidate.tmdb_id.to_string());

    if let Some(language) = &candidate.language {
        push_tag(&mut xml, "language", language);
    }

    if let Some(vote_average) = candidate.vote_average {
        push_tag(&mut xml, "rating", &format!("{:.1}", vote_average));
    }

    if let Some(poster_path) = &candidate.poster_path {
        push_tag(&mut xml, "thumb", &tmdb_image_url(poster_path));
    }

    if let Some(backdrop_path) = &candidate.backdrop_path {
        xml.push_str("  <fanart>\n");
        push_nested_tag(&mut xml, "thumb", &tmdb_image_url(backdrop_path));
        xml.push_str("  </fanart>\n");
    }

    xml.push_str(&format!("</{}>\n", root));
    xml
}

pub fn tmdb_image_url(path: &str) -> String {
    if path.starts_with("http://") || path.starts_with("https://") {
        return path.to_string();
    }

    let normalized = if path.starts_with('/') {
        path.to_string()
    } else {
        format!("/{}", path)
    };

    format!("https://image.tmdb.org/t/p/original{}", normalized)
}

fn push_tag(xml: &mut String, tag: &str, value: &str) {
    xml.push_str(&format!("  <{}>{}</{}>\n", tag, escape_xml(value), tag));
}

fn push_nested_tag(xml: &mut String, tag: &str, value: &str) {
    xml.push_str(&format!("    <{}>{}</{}>\n", tag, escape_xml(value), tag));
}

fn escape_xml(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tmdb_search_contract::TmdbSearchMediaType;

    fn make_candidate() -> TmdbCandidate {
        TmdbCandidate {
            id: "tmdb-1".to_string(),
            tmdb_id: 1,
            title: "A & B".to_string(),
            original_title: Some("Original".to_string()),
            media_type: TmdbSearchMediaType::Movie,
            release_year: Some(2020),
            overview: Some("Plot <test>".to_string()),
            poster_path: Some("/poster.jpg".to_string()),
            backdrop_path: None,
            language: Some("en".to_string()),
            popularity: None,
            vote_average: Some(7.5),
            confidence_hint: None,
            match_reasons: vec![],
        }
    }

    #[test]
    fn builds_movie_nfo_with_escaped_xml() {
        let nfo = build_nfo_document(&make_candidate());

        assert!(nfo.contains("<movie>"));
        assert!(nfo.contains("A &amp; B"));
        assert!(nfo.contains("Plot &lt;test&gt;"));
        assert!(nfo.contains("https://image.tmdb.org/t/p/original/poster.jpg"));
    }

    #[test]
    fn builds_absolute_tmdb_image_url() {
        assert_eq!(
            tmdb_image_url("/poster.jpg"),
            "https://image.tmdb.org/t/p/original/poster.jpg"
        );
    }
}
