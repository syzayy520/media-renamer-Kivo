// 审计事件模块
// 职责：定义执行相关的审计事件类型
// 不做审计写入，不做执行逻辑

use serde::Serialize;

/// 审计事件类型
#[derive(Debug, Clone, Serialize)]
pub enum AuditEvent {
    /// 执行计划生成
    ExecutionPlanCreated {
        task_id: String,
        file_count: usize,
        mode: String,
    },
    /// 安全门闸检查
    SafetyGateChecked {
        task_id: String,
        passed: bool,
        blocking_reasons: Vec<String>,
    },
    /// 用户确认
    UserConfirmed { task_id: String, mode: String },
    /// 执行开始
    ExecutionStarted {
        task_id: String,
        mode: String,
        total_files: usize,
    },
    /// 单项执行成功
    ItemExecuted {
        task_id: String,
        source: String,
        target: String,
        success: bool,
        error: Option<String>,
    },
    /// 执行完成
    ExecutionCompleted {
        task_id: String,
        success_count: usize,
        failed_count: usize,
        skipped_count: usize,
        blocked_count: usize,
    },
    /// 回滚计划生成
    RollbackPlanCreated { task_id: String, entry_count: usize },
    /// 回滚执行
    RollbackExecuted {
        task_id: String,
        success_count: usize,
        failed_count: usize,
        blocked_count: usize,
    },
}

impl AuditEvent {
    /// 获取事件类型名称
    pub fn event_type(&self) -> &'static str {
        match self {
            AuditEvent::ExecutionPlanCreated { .. } => "execution_plan_created",
            AuditEvent::SafetyGateChecked { .. } => "safety_gate_checked",
            AuditEvent::UserConfirmed { .. } => "user_confirmed",
            AuditEvent::ExecutionStarted { .. } => "execution_started",
            AuditEvent::ItemExecuted { .. } => "item_executed",
            AuditEvent::ExecutionCompleted { .. } => "execution_completed",
            AuditEvent::RollbackPlanCreated { .. } => "rollback_plan_created",
            AuditEvent::RollbackExecuted { .. } => "rollback_executed",
        }
    }

    /// 获取任务 ID
    pub fn task_id(&self) -> &str {
        match self {
            AuditEvent::ExecutionPlanCreated { task_id, .. }
            | AuditEvent::SafetyGateChecked { task_id, .. }
            | AuditEvent::UserConfirmed { task_id, .. }
            | AuditEvent::ExecutionStarted { task_id, .. }
            | AuditEvent::ItemExecuted { task_id, .. }
            | AuditEvent::ExecutionCompleted { task_id, .. }
            | AuditEvent::RollbackPlanCreated { task_id, .. }
            | AuditEvent::RollbackExecuted { task_id, .. } => task_id,
        }
    }

    /// 转换为可序列化的审计日志消息
    pub fn to_log_message(&self) -> String {
        match self {
            AuditEvent::ExecutionPlanCreated {
                task_id,
                file_count,
                mode,
            } => {
                format!(
                    "Execution plan created for task {}: {} files in {} mode",
                    task_id, file_count, mode
                )
            }
            AuditEvent::SafetyGateChecked {
                task_id,
                passed,
                blocking_reasons,
            } => {
                format!(
                    "Safety gate check for task {}: {}{}",
                    task_id,
                    if *passed { "PASSED" } else { "FAILED" },
                    if blocking_reasons.is_empty() {
                        String::new()
                    } else {
                        format!(" - {}", blocking_reasons.join("; "))
                    }
                )
            }
            AuditEvent::UserConfirmed { task_id, mode } => {
                format!(
                    "User confirmed execution for task {} in {} mode",
                    task_id, mode
                )
            }
            AuditEvent::ExecutionStarted {
                task_id,
                mode,
                total_files,
            } => {
                format!(
                    "Execution started for task {}: {} files in {} mode",
                    task_id, total_files, mode
                )
            }
            AuditEvent::ItemExecuted {
                task_id,
                source,
                target,
                success,
                error,
            } => {
                format!(
                    "Item executed for task {}: {} -> {} {}{}",
                    task_id,
                    source,
                    target,
                    if *success { "SUCCESS" } else { "FAILED" },
                    error
                        .as_ref()
                        .map(|e| format!(" - {}", e))
                        .unwrap_or_default()
                )
            }
            AuditEvent::ExecutionCompleted {
                task_id,
                success_count,
                failed_count,
                skipped_count,
                blocked_count,
            } => {
                format!(
                    "Execution completed for task {}: {} success, {} failed, {} skipped, {} blocked",
                    task_id, success_count, failed_count, skipped_count, blocked_count
                )
            }
            AuditEvent::RollbackPlanCreated {
                task_id,
                entry_count,
            } => {
                format!(
                    "Rollback plan created for task {}: {} entries",
                    task_id, entry_count
                )
            }
            AuditEvent::RollbackExecuted {
                task_id,
                success_count,
                failed_count,
                blocked_count,
            } => {
                format!(
                    "Rollback executed for task {}: {} success, {} failed, {} blocked",
                    task_id, success_count, failed_count, blocked_count
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audit_event_type_names() {
        let event = AuditEvent::ExecutionPlanCreated {
            task_id: "test".to_string(),
            file_count: 5,
            mode: "DryRun".to_string(),
        };
        assert_eq!(event.event_type(), "execution_plan_created");
        assert_eq!(event.task_id(), "test");
    }

    #[test]
    fn test_audit_event_log_message() {
        let event = AuditEvent::SafetyGateChecked {
            task_id: "test".to_string(),
            passed: true,
            blocking_reasons: vec![],
        };
        let message = event.to_log_message();
        assert!(message.contains("PASSED"));
        assert!(message.contains("test"));
    }

    #[test]
    fn test_audit_event_log_message_with_reasons() {
        let event = AuditEvent::SafetyGateChecked {
            task_id: "test".to_string(),
            passed: false,
            blocking_reasons: vec!["low confidence".to_string()],
        };
        let message = event.to_log_message();
        assert!(message.contains("FAILED"));
        assert!(message.contains("low confidence"));
    }

    #[test]
    fn test_execution_completed_message() {
        let event = AuditEvent::ExecutionCompleted {
            task_id: "test".to_string(),
            success_count: 3,
            failed_count: 1,
            skipped_count: 2,
            blocked_count: 0,
        };
        let message = event.to_log_message();
        assert!(message.contains("3 success"));
        assert!(message.contains("1 failed"));
        assert!(message.contains("2 skipped"));
    }
}
