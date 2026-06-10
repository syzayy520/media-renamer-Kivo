// src-tauri/src/tmdb_search/service/parse_search_response.rs
// 职责：解析 TMDb live search JSON，并映射为前端候选列表

use crate::tmdb_search::mapping::candidate_mapper::map_to_candidates;
use crate::tmdb_search::response::tmdb_search_response::TmdbSearchResponse;
use crate::tmdb_search_contract::{
    SearchTmdbCandidatesOutput, TmdbCandidateSource, TmdbSearchError, TmdbSearchErrorCode,
    TmdbSearchMediaType,
};

pub fn parse_search_response(
    response: &str,
    media_type: TmdbSearchMediaType,
) -> SearchTmdbCandidatesOutput {
    match serde_json::from_str::<TmdbSearchResponse>(response) {
        Ok(parsed) => SearchTmdbCandidatesOutput {
            candidates: map_to_candidates(&parsed.results, media_type),
            source: TmdbCandidateSource::Tmdb,
            rate_limit: None,
            error: None,
        },
        Err(_) => SearchTmdbCandidatesOutput {
            candidates: vec![],
            source: TmdbCandidateSource::Tmdb,
            rate_limit: None,
            error: Some(TmdbSearchError {
                code: TmdbSearchErrorCode::Unknown,
                message: "Failed to parse TMDb search response.".to_string(),
                retryable: false,
                retry_after: None,
            }),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_movie_search_results_into_candidates() {
        let response = r#"
        {
          "page": 1,
          "total_pages": 1,
          "total_results": 1,
          "results": [
            {
              "id": 11152,
              "title": "The Ruins",
              "original_title": "The Ruins",
              "release_date": "2008-04-02",
              "overview": "A group of friends discover ancient ruins.",
              "poster_path": "/poster.jpg",
              "backdrop_path": "/backdrop.jpg",
              "original_language": "en",
              "popularity": 25.4,
              "vote_average": 5.9
            }
          ]
        }
        "#;

        let output = parse_search_response(response, TmdbSearchMediaType::Movie);

        assert!(output.error.is_none());
        assert_eq!(output.candidates.len(), 1);
        assert_eq!(output.candidates[0].title, "The Ruins");
        assert_eq!(output.candidates[0].release_year, Some(2008));
        assert_eq!(output.candidates[0].poster_path.as_deref(), Some("/poster.jpg"));
    }

    #[test]
    fn returns_empty_candidates_for_valid_empty_response() {
        let response = r#"
        {
          "page": 1,
          "total_pages": 1,
          "total_results": 0,
          "results": []
        }
        "#;

        let output = parse_search_response(response, TmdbSearchMediaType::Movie);

        assert!(output.error.is_none());
        assert!(output.candidates.is_empty());
    }

    #[test]
    fn invalid_json_returns_structured_error() {
        let output = parse_search_response("not-json", TmdbSearchMediaType::Movie);

        assert!(output.candidates.is_empty());
        assert!(output.error.is_some());
        assert_eq!(output.error.unwrap().code, TmdbSearchErrorCode::Unknown);
    }
}
