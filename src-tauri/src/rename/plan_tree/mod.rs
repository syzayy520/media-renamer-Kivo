//! 预览计划树模块 — Folder-aware 目录树数据结构
//!
//! 替代平铺文件表，升级为文件夹组 + 子文件结构。
//!
//! 子模块:
//! - folder_group: 文件夹组数据结构
//! - file_item: 子文件项数据结构
//! - group_status: 组状态聚合
//! - group_aggregation: 从平铺预览项构建目录树

pub mod file_item;
pub mod folder_group;
pub mod group_aggregation;
pub mod group_status;

pub use file_item::*;
pub use folder_group::*;
pub use group_aggregation::*;
pub use group_status::*;
