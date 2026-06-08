// api/audit/getTask.ts — get_task 命令封装
// 职责：获取单个任务

import { invokeCommand } from '../invoke';
import type { RenameTask } from './types';

export async function getTask(taskId: string): Promise<RenameTask> {
  return invokeCommand<RenameTask>('get_task', { taskId });
}
