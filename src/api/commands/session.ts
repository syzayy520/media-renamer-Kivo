// api/commands/session.ts — Session 命令封装
// 职责：封装 start_rename_session Tauri 命令

import { invokeCommand } from '../invoke';
import type { PipelineResult, StartRenameSessionInput } from '../types';

/**
 * 启动重命名计划会话（Dry-run）
 *
 * 流程：扫描目录 → 解析文件名 → 生成预览 → 安全检查 → 写审计记录
 * 不会真实修改媒体文件。
 *
 * @param input - 目录路径
 * @returns PipelineResult 扫描结果、预览列表、安全检查报告
 */
export async function startRenameSession(
  input: StartRenameSessionInput,
): Promise<PipelineResult> {
  return invokeCommand<PipelineResult>('start_rename_session', {
    directory: input.directory,
  });
}
