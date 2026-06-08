// api/invoke.ts — Tauri invoke 统一封装
// 职责：包装 @tauri-apps/api/core 的 invoke，规范化错误处理

import { invoke as tauriInvoke } from '@tauri-apps/api/core';
import type { ApiError } from './types';

/**
 * 调用 Tauri 后端命令
 *
 * @param command - Tauri 命令名
 * @param args - 命令参数
 * @returns Promise<T> 返回类型化的结果
 * @throws ApiError 统一错误格式
 */
export async function invokeCommand<T>(
  command: string,
  args?: Record<string, unknown>,
): Promise<T> {
  try {
    return await tauriInvoke<T>(command, args);
  } catch (err: unknown) {
    const message =
      err instanceof Error ? err.message : String(err);
    throw { message } satisfies ApiError;
  }
}
