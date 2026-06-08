// api/config/confidenceThreshold.ts — 置信度阈值命令封装
// 职责：get_confidence_threshold 和 set_confidence_threshold

import { invokeCommand } from '../invoke';

export async function getConfidenceThreshold(): Promise<number> {
  return invokeCommand<number>('get_confidence_threshold');
}

export async function setConfidenceThreshold(value: number): Promise<void> {
  return invokeCommand<void>('set_confidence_threshold', { value });
}
