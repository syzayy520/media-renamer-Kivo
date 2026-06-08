// api/audit/getAllTasks.ts — get_all_tasks 命令封装
// 职责：获取所有任务列表

import { invokeCommand } from '../invoke';
import type { RenameTask } from './types';

export async function getAllTasks(): Promise<RenameTask[]> {
  return invokeCommand<RenameTask[]>('get_all_tasks');
}
