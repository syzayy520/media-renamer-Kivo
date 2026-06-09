// api/invoke.ts — Tauri invoke 统一封装
// 职责：包装 @tauri-apps/api/core 的 invoke，规范化错误处理

import { invoke as tauriInvoke } from '@tauri-apps/api/core';

/**
 * 从 unknown error 中提取可读消息
 */
export function toErrorMessage(err: unknown): string {
  if (typeof err === 'string') return err;
  if (err instanceof Error) return err.message;
  if (err && typeof err === 'object') {
    const obj = err as Record<string, unknown>;
    if (typeof obj.message === 'string') return obj.message;
    if (typeof obj.error === 'string') return obj.error;
    if (typeof obj.reason === 'string') return obj.reason;
    try {
      return JSON.stringify(err);
    } catch {
      // fall through
    }
  }
  return 'Unknown error';
}

/**
 * 调用 Tauri 后端命令
 */
export async function invokeCommand<T>(
  command: string,
  args?: Record<string, unknown>,
): Promise<T> {
  try {
    return await tauriInvoke<T>(command, args);
  } catch (err: unknown) {
    throw new Error(toErrorMessage(err), { cause: err });
  }
}
