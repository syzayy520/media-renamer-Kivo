// api/config/getAllTemplates.ts — get_all_templates 命令封装
// 职责：获取所有重命名模板

import { invokeCommand } from '../invoke';
import type { RenameRule } from './types';

export async function getAllTemplates(): Promise<RenameRule[]> {
  return invokeCommand<RenameRule[]>('get_all_templates');
}
