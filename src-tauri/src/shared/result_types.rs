// 结果类型模块
// 职责：定义统一的错误类型和结果类型

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// 应用结果类型
pub type AppResult<T> = Result<T, AppError>;

/// 应用错误类型
#[derive(Error, Debug, Serialize, Deserialize)]
pub enum AppError {
    // 基础错误
    #[error("IO error: {0}")]
    Io(String),

    #[error("Database error: {0}")]
    Database(String),

    #[error("Serialization error: {0}")]
    Serialization(String),

    // 扫描错误
    #[error("Permission denied: {path}")]
    PermissionDenied { path: String },

    #[error("Path too long ({length} > {max}): {path}")]
    PathTooLong {
        path: String,
        length: usize,
        max: usize,
    },

    #[error("Circular symlink detected: {path}")]
    CircularSymlink { path: String },

    #[error("Max file limit reached: {limit}")]
    MaxFileLimitReached { limit: u32 },

    // 解析错误
    #[error("Parse failed: {reason}")]
    ParseFailed { reason: String },

    #[error("Unknown media format: {filename}")]
    UnknownFormat { filename: String },

    // 重命名错误
    #[error("Conflict: {conflict_type}: {message}")]
    Conflict {
        conflict_type: ConflictType,
        message: String,
    },

    #[error("Safety check failed: {check_name}: {reason}")]
    SafetyCheckFailed { check_name: String, reason: String },

    #[error("Rename failed: {before} -> {after}: {reason}")]
    RenameFailed {
        before: String,
        after: String,
        reason: String,
    },

    // 回滚错误
    #[error("Rollback conflict: {message}")]
    RollbackConflict { message: String },

    #[error("Source file not found: {path}")]
    SourceNotFound { path: String },

    #[error("Target already exists: {path}")]
    TargetAlreadyExists { path: String },

    // 配置错误
    #[error("Config error: {0}")]
    Config(String),

    // 任务错误
    #[error("Task not found: {task_id}")]
    TaskNotFound { task_id: String },

    #[error("Task not in rollbackable state: {status}")]
    TaskNotRollbackable { status: String },

    // 通用错误
    #[error("Unknown error: {0}")]
    Unknown(String),
}

/// 冲突类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConflictType {
    TargetExists,
    DuplicateTarget,
    PathTooLong,
    InvalidChars,
    PermissionDenied,
    SourceNotFound,
}

impl std::fmt::Display for ConflictType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConflictType::TargetExists => write!(f, "TargetExists"),
            ConflictType::DuplicateTarget => write!(f, "DuplicateTarget"),
            ConflictType::PathTooLong => write!(f, "PathTooLong"),
            ConflictType::InvalidChars => write!(f, "InvalidChars"),
            ConflictType::PermissionDenied => write!(f, "PermissionDenied"),
            ConflictType::SourceNotFound => write!(f, "SourceNotFound"),
        }
    }
}

// 实现从 std::io::Error 的转换
impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::Io(err.to_string())
    }
}

// 实现从 rusqlite::Error 的转换
impl From<rusqlite::Error> for AppError {
    fn from(err: rusqlite::Error) -> Self {
        AppError::Database(err.to_string())
    }
}

// 实现从 serde_json::Error 的转换
impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::Serialization(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = AppError::PermissionDenied {
            path: "/test/path".to_string(),
        };
        assert!(err.to_string().contains("Permission denied"));
    }

    #[test]
    fn test_conflict_type_display() {
        assert_eq!(ConflictType::TargetExists.to_string(), "TargetExists");
    }
}
