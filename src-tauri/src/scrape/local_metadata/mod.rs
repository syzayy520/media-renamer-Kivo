// src-tauri/src/scrape/local_metadata/mod.rs
// 本地元数据刮削子家族入口
// 职责：组合 scrape input/output、NFO 构建、图片下载、落盘执行模块

pub mod image_asset_downloader;
pub mod local_scrape_input;
pub mod local_scrape_output;
pub mod local_metadata_writer;
pub mod nfo_document_builder;
pub mod scrape_to_local_executor;

pub use local_scrape_input::LocalScrapeInput;
pub use local_scrape_output::{LocalScrapeOutput, LocalScrapeWrittenFile};
pub use scrape_to_local_executor::scrape_to_local;
