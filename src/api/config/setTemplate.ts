// api/config/setTemplate.ts — set_template 命令封装
// 职责：设置指定媒体类型的模板

import { invokeCommand } from '../invoke';

export async function setTemplate(
  mediaType: string,
  template: string,
): Promise<void> {
  return invokeCommand<void>('set_template', { mediaType, template });
}
