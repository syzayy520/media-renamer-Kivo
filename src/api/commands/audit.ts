// api/commands/audit.ts — Audit 命令封装
// 职责：封装 audit 相关 Tauri 命令

import { invokeCommand } from '../invoke';
import type { RenameTask, AuditLogEntry } from '../types';

/**
 * 获取指定任务
 *
 * @param taskId - 任务 ID
 */
export async function getTask(taskId: string): Promise<RenameTask> {
  return invokeCommand<RenameTask>('get_task', { taskId });
}

/**
 * 获取所有任务（按创建时间倒序）
 */
export async function getAllTasks(): Promise<RenameTask[]> {
  return invokeCommand<RenameTask[]>('get_all_tasks');
}

/**
 * 获取指定任务的审计日志
 *
 * @param taskId - 任务 ID
 */
export async function getAuditLogs(
  taskId: string,
): Promise<AuditLogEntry[]> {
  return invokeCommand<AuditLogEntry[]>('get_audit_logs', { taskId });
}

// === Deferred Commands (Not Exposed) ===
// execute_rename: CORE READY / EXPOSURE DEFERRED
// rollback_task: CORE READY / EXPOSURE DEFERRED
// 不在本模块中导出 wrapper。
