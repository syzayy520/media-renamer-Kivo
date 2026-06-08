// api/commands/config.ts — Config 命令封装
// 职责：封装 config 相关 Tauri 命令

import { invokeCommand } from '../invoke';
import type { AppConfig, RenameRule } from '../types';

/**
 * 获取完整应用配置
 */
export async function getAppConfig(): Promise<AppConfig> {
  return invokeCommand<AppConfig>('get_app_config');
}

/**
 * 获取所有重命名模板
 */
export async function getAllTemplates(): Promise<RenameRule[]> {
  return invokeCommand<RenameRule[]>('get_all_templates');
}

/**
 * 设置指定媒体类型的模板
 *
 * @param mediaType - 媒体类型字符串 ("movie" | "series" | "anime" | "special" | "ova" | "extras" | "unknown")
 * @param template - 模板字符串
 */
export async function setTemplate(
  mediaType: string,
  template: string,
): Promise<void> {
  return invokeCommand<void>('set_template', {
    mediaType,
    template,
  });
}

/**
 * 获取置信度阈值
 */
export async function getConfidenceThreshold(): Promise<number> {
  return invokeCommand<number>('get_confidence_threshold');
}

/**
 * 设置置信度阈值
 *
 * @param value - 阈值 (0-100)
 */
export async function setConfidenceThreshold(value: number): Promise<void> {
  return invokeCommand<void>('set_confidence_threshold', { value });
}
