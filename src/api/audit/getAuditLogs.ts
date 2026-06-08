// api/audit/getAuditLogs.ts — get_audit_logs 命令封装
// 职责：获取审计日志

import { invokeCommand } from '../invoke';
import type { AuditLogEntry } from './types';

export async function getAuditLogs(taskId: string): Promise<AuditLogEntry[]> {
  return invokeCommand<AuditLogEntry[]>('get_audit_logs', { taskId });
}
