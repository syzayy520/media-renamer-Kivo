// api/index.ts — API 模块统一导出
// 职责：registry / re-export only

// Types
export type * from './types';

// Invoke wrapper
export { invokeCommand } from './invoke';

// Commands
export { startRenameSession } from './commands/session';
export {
  getAppConfig,
  getAllTemplates,
  setTemplate,
  getConfidenceThreshold,
  setConfidenceThreshold,
} from './commands/config';
export { getTask, getAllTasks, getAuditLogs } from './commands/audit';
