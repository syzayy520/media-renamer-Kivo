use serde::Deserialize;

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq)]
pub enum LocalScrapeMode {
    BesideCurrentMedia,
    IntoPreparedFolder,
}

impl Default for LocalScrapeMode {
    fn default() -> Self {
        Self::BesideCurrentMedia
    }
}
