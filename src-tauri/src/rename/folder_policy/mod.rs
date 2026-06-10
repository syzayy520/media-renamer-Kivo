//! 文件夹策略模块 — 决定整理后的目录结构
//!
//! 支持 7 种策略:
//! 1. KeepOriginalStructure  保持原结构
//! 2. OneMovieOneFolder      一片一夹
//! 3. NormalizeExisting      规范已有文件夹
//! 4. ChineseFolderPtFile    中文文件夹+PT文件名
//! 5. Flatten                去除文件夹
//! 6. TvShowStructure        剧集目录结构
//! 7. NoFolder               不建文件夹

pub mod folder_operation;
pub mod policy;

pub use folder_operation::*;
pub use policy::*;
