use std::marker::PhantomData;
use std::sync::Arc;

use crate::tmdb_search::transport::TransportGate;
use crate::tmdb_search_contract::{SearchTmdbCandidatesInput, SearchTmdbCandidatesOutput};

pub struct LiveSearchService<P> {
    _gate: Arc<TransportGate>,
    _provider: PhantomData<P>,
}

impl<P> LiveSearchService<P> {
    pub fn new(_provider: P, gate: Arc<TransportGate>) -> Self {
        Self {
            _gate: gate,
            _provider: PhantomData,
        }
    }

    pub async fn search(&self, _input: SearchTmdbCandidatesInput) -> SearchTmdbCandidatesOutput {
        todo!("live search parser repair in progress")
    }
}
