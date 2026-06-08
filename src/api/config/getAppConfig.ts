// api/config/getAppConfig.ts — get_app_config 命令封装
// 职责：获取完整应用配置

import { invokeCommand } from '../invoke';
import type { AppConfig } from './types';

export async function getAppConfig(): Promise<AppConfig> {
  return invokeCommand<AppConfig>('get_app_config');
}
