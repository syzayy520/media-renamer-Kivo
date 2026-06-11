pub mod image_asset_downloader;
pub mod local_metadata_writer;
pub mod local_scrape_input;
pub mod local_scrape_mode;
pub mod local_scrape_output;
pub mod nfo_document_builder;
pub mod scrape_file_name_plan;
pub mod scrape_target_folder_resolver;
pub mod scrape_to_local_executor;

pub use local_scrape_input::LocalScrapeInput;
pub use local_scrape_output::{LocalScrapeOutput, LocalScrapeWrittenFile};
pub use scrape_to_local_executor::scrape_to_local;
