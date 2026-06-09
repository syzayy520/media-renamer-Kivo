// shared/ui/Badge 模块 - 标签组件
// 职责：提供语义化状态标签，用于安全状态指示

import type { ReactNode } from 'react';

type BadgeVariant = 'success' | 'warning' | 'danger' | 'info';

interface BadgeProps {
  variant?: BadgeVariant;
  children: ReactNode;
}

const variantStyles: Record<BadgeVariant, string> = {
  success: 'bg-emerald-500/15 text-emerald-400 border-emerald-500/30',
  warning: 'bg-amber-500/15 text-amber-400 border-amber-500/30',
  danger: 'bg-red-500/15 text-red-400 border-red-500/30',
  info: 'bg-blue-500/15 text-blue-400 border-blue-500/30',
};

export function Badge({ variant = 'info', children }: BadgeProps) {
  return (
    <span
      className={`inline-flex items-center rounded border px-2 py-0.5 text-xs font-medium ${variantStyles[variant] ?? variantStyles.info}`}
    >
      {children}
    </span>
  );
}
