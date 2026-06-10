use std::marker::PhantomData;
use std::sync::Arc;

use crate::tmdb_search::transport::TransportGate;
use crate::tmdb_search_contract::{SearchTmdbCandidatesInput, SearchTmdbCandidatesOutput, TmdbCandidateSource};

pub struct LiveSearchService<P> {
    gate: Arc<TransportGate>,
    _provider: PhantomData<P>,
}

impl<P> LiveSearchService<P> {
    pub fn new(_provider: P, gate: Arc<TransportGate>) -> Self {
        Self {
            gate,
            _provider: PhantomData,
        }
    }

    pub async fn search(&self, _input: SearchTmdbCandidatesInput) -> SearchTmdbCandidatesOutput {
        let _enabled = self.gate.is_enabled();
        SearchTmdbCandidatesOutput {
            candidates: vec![],
            source: TmdbCandidateSource::Tmdb,
            rate_limit: None,
            error: None,
        }
    }
}
