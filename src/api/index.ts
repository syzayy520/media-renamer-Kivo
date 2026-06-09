// api/index.ts — API 模块统一导出
// 职责：registry / re-export only

// Types
export type * from './shared/apiError';
export type * from './session/types';
export type * from './config/types';
export type * from './audit/types';

// Invoke wrapper
export { invokeCommand } from './invoke';

// Session
export { startRenameSession } from './session/startRenameSession';

// Config
export { getAppConfig } from './config/getAppConfig';
export { getAllTemplates } from './config/getAllTemplates';
export { setTemplate } from './config/setTemplate';
export {
  getConfidenceThreshold,
  setConfidenceThreshold,
} from './config/confidenceThreshold';
export {
  getTmdbApiKeyStatus,
  setTmdbApiKey,
  clearTmdbApiKey,
} from './config/tmdbApiKey';

// Audit
export { getTask } from './audit/getTask';
export { getAllTasks } from './audit/getAllTasks';
export { getAuditLogs } from './audit/getAuditLogs';
