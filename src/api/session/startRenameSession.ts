// api/session/startRenameSession.ts — start_rename_session 命令封装
// 职责：封装扫描会话命令

import { invokeCommand } from '../invoke';
import type { PipelineResult, StartRenameSessionInput } from './types';

export async function startRenameSession(
  input: StartRenameSessionInput,
): Promise<PipelineResult> {
  return invokeCommand<PipelineResult>('start_rename_session', {
    directory: input.directory,
  });
}
