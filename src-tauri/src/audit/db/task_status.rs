// 任务状态枚举模块
// 职责：定义 TaskStatus 枚举及其转换逻辑
// 不做 DB 操作，不做 schema 定义

use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// 重命名任务状态
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TaskStatus {
    /// 预览生成中
    Previewing,
    /// 等待执行
    Pending,
    /// 执行中
    Executing,
    /// 已完成
    Completed,
    /// 失败
    Failed,
    /// 已回滚
    RolledBack,
}

impl std::fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskStatus::Previewing => write!(f, "Previewing"),
            TaskStatus::Pending => write!(f, "Pending"),
            TaskStatus::Executing => write!(f, "Executing"),
            TaskStatus::Completed => write!(f, "Completed"),
            TaskStatus::Failed => write!(f, "Failed"),
            TaskStatus::RolledBack => write!(f, "RolledBack"),
        }
    }
}

impl std::str::FromStr for TaskStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Previewing" => Ok(TaskStatus::Previewing),
            "Pending" => Ok(TaskStatus::Pending),
            "Executing" => Ok(TaskStatus::Executing),
            "Completed" => Ok(TaskStatus::Completed),
            "Failed" => Ok(TaskStatus::Failed),
            "RolledBack" => Ok(TaskStatus::RolledBack),
            _ => Err(format!("Invalid task status: {}", s)),
        }
    }
}

/// 解析任务状态字符串（兼容旧代码）
pub fn parse_task_status(s: &str) -> TaskStatus {
    TaskStatus::from_str(s).unwrap_or(TaskStatus::Failed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_display() {
        assert_eq!(TaskStatus::Previewing.to_string(), "Previewing");
        assert_eq!(TaskStatus::Pending.to_string(), "Pending");
        assert_eq!(TaskStatus::Executing.to_string(), "Executing");
        assert_eq!(TaskStatus::Completed.to_string(), "Completed");
        assert_eq!(TaskStatus::Failed.to_string(), "Failed");
        assert_eq!(TaskStatus::RolledBack.to_string(), "RolledBack");
    }

    #[test]
    fn test_status_parse() {
        assert_eq!(
            TaskStatus::from_str("Previewing").unwrap(),
            TaskStatus::Previewing
        );
        assert_eq!(
            TaskStatus::from_str("Completed").unwrap(),
            TaskStatus::Completed
        );
        assert_eq!(
            TaskStatus::from_str("RolledBack").unwrap(),
            TaskStatus::RolledBack
        );
    }

    #[test]
    fn test_invalid_status_handling() {
        // parse_task_status 对无效输入返回 Failed
        assert_eq!(parse_task_status("Invalid"), TaskStatus::Failed);
        assert_eq!(parse_task_status(""), TaskStatus::Failed);

        // from_str 对无效输入返回 Err
        assert!(TaskStatus::from_str("Invalid").is_err());
    }

    #[test]
    fn test_parse_task_status_valid() {
        assert_eq!(parse_task_status("Previewing"), TaskStatus::Previewing);
        assert_eq!(parse_task_status("Pending"), TaskStatus::Pending);
        assert_eq!(parse_task_status("Executing"), TaskStatus::Executing);
        assert_eq!(parse_task_status("Completed"), TaskStatus::Completed);
        assert_eq!(parse_task_status("Failed"), TaskStatus::Failed);
        assert_eq!(parse_task_status("RolledBack"), TaskStatus::RolledBack);
    }
}
