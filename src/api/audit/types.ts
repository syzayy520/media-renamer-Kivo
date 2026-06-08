// api/audit/types.ts — Audit 相关类型
// 职责：RenameTask / AuditLogEntry / TaskStatus 等

export type TaskStatus =
  | 'Previewing'
  | 'Pending'
  | 'Executing'
  | 'Completed'
  | 'Failed'
  | 'RolledBack';

export interface RenameTask {
  id: string;
  status: TaskStatus;
  template: string;
  total_files: number;
  created_at: string;
  updated_at: string;
  error_message: string | null;
}

export interface AuditLogEntry {
  id: string;
  task_id: string | null;
  event_type: string;
  message: string;
  created_at: string;
}
