use std::sync::Arc;

use crate::tmdb_search::api_key::TmdbApiKeyProvider;
use crate::tmdb_search::service::parse_search_response::parse_search_response;
use crate::tmdb_search::transport::{LiveHttpTransport, TmdbTransport, TransportGate};
use crate::tmdb_search_contract::{
    SearchTmdbCandidatesInput, SearchTmdbCandidatesOutput, TmdbCandidateSource, TmdbSearchMediaType,
};

pub struct LiveSearchService<P: TmdbApiKeyProvider> {
    provider: P,
    gate: Arc<TransportGate>,
}

impl<P: TmdbApiKeyProvider> LiveSearchService<P> {
    pub fn new(provider: P, gate: Arc<TransportGate>) -> Self {
        Self { provider, gate }
    }

    pub async fn search(&self, input: SearchTmdbCandidatesInput) -> SearchTmdbCandidatesOutput {
        if !self.gate.is_enabled() {
            return empty_output();
        }

        let Some(key) = self.provider.get_api_key().await else {
            return empty_output();
        };

        let media_type = input.media_type;
        let media_type_str = match media_type {
            TmdbSearchMediaType::Movie => "movie",
            TmdbSearchMediaType::Tv => "tv",
        };
        let path = format!("/search/{}", media_type_str);
        let mut params = vec![
            ("query".to_string(), input.query),
            ("language".to_string(), input.language),
        ];

        if let Some(year) = input.year {
            let year_key = match media_type {
                TmdbSearchMediaType::Movie => "year",
                TmdbSearchMediaType::Tv => "first_air_date_year",
            };
            params.push((year_key.to_string(), year.to_string()));
        }

        if let Some(page) = input.page {
            params.push(("page".to_string(), page.to_string()));
        }

        let transport = LiveHttpTransport::new(key, self.gate.clone());

        match transport.send_search_request(&path, &params).await {
            Ok(response) => parse_search_response(&response, media_type),
            Err(_) => empty_output(),
        }
    }
}

fn empty_output() -> SearchTmdbCandidatesOutput {
    SearchTmdbCandidatesOutput {
        candidates: vec![],
        source: TmdbCandidateSource::Tmdb,
        rate_limit: None,
        error: None,
    }
}
