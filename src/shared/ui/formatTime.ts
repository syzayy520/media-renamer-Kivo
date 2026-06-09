// shared/ui/formatTime.ts — 时间格式化工具
// 职责：日期时间格式化

export function formatTime(iso: string, format: 'short' | 'full' = 'short'): string {
  try {
    if (format === 'full') {
      return new Date(iso).toLocaleString('zh-CN');
    }
    return new Date(iso).toLocaleString('zh-CN', {
      hour: '2-digit',
      minute: '2-digit',
      second: '2-digit',
    });
  } catch {
    return format === 'full' ? iso.replace('T', ' ').slice(0, 19) : iso.slice(11, 19);
  }
}

export function formatTaskTime(iso: string): string {
  try {
    return new Date(iso).toLocaleString('zh-CN', {
      month: '2-digit',
      day: '2-digit',
      hour: '2-digit',
      minute: '2-digit',
    });
  } catch {
    return iso.slice(0, 16);
  }
}
