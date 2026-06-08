// 单条回滚模块
// 职责：执行单条结果的回滚操作
// 不做任务级检查，不做统计聚合

use super::rollback_entry::{RollbackEntry, RollbackStatus};
use crate::audit::db::{RenameResult, TaskStatus};
use crate::audit::logger::AuditLogger;

/// 回滚单条结果
pub fn rollback_single(logger: &AuditLogger, result: &RenameResult) -> RollbackEntry {
    // 只有 Completed 状态可回滚
    if result.status != TaskStatus::Completed {
        return RollbackEntry {
            rename_result_id: result.id.clone(),
            status: RollbackStatus::Blocked,
            error_message: Some(format!("Status is {}, not Completed", result.status)),
        };
    }

    let target = std::path::Path::new(&result.target_path);
    let source = std::path::Path::new(&result.source_path);

    // 检查 afterPath 是否存在
    if !target.exists() {
        let _ = logger.log_failure(
            Some(&result.task_id),
            &format!(
                "Rollback blocked: afterPath missing: {}",
                result.target_path
            ),
        );
        return RollbackEntry {
            rename_result_id: result.id.clone(),
            status: RollbackStatus::Blocked,
            error_message: Some(format!("afterPath does not exist: {}", result.target_path)),
        };
    }

    // 检查 beforePath 是否被占用
    if source.exists() {
        let _ = logger.log_failure(
            Some(&result.task_id),
            &format!(
                "Rollback blocked: beforePath occupied: {}",
                result.source_path
            ),
        );
        return RollbackEntry {
            rename_result_id: result.id.clone(),
            status: RollbackStatus::Blocked,
            error_message: Some(format!(
                "beforePath already occupied: {}",
                result.source_path
            )),
        };
    }

    // 执行回滚 rename (target -> source)
    match std::fs::rename(target, source) {
        Ok(()) => {
            let _ = logger.log_event(
                Some(&result.task_id),
                "rollback_success",
                &format!(
                    "Rolled back: {} -> {}",
                    result.target_path, result.source_path
                ),
            );
            RollbackEntry {
                rename_result_id: result.id.clone(),
                status: RollbackStatus::Success,
                error_message: None,
            }
        }
        Err(e) => {
            let _ = logger.log_failure(
                Some(&result.task_id),
                &format!("Rollback failed: {}: {}", result.target_path, e),
            );
            RollbackEntry {
                rename_result_id: result.id.clone(),
                status: RollbackStatus::Failed,
                error_message: Some(format!("Rename failed: {}", e)),
            }
        }
    }
}
