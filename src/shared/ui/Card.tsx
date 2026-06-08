// shared/ui/Card 模块 - 基础卡片组件
// 职责：提供统一的卡片容器，用于 placeholder 页面和未来的功能组件

import type { HTMLAttributes, ReactNode } from 'react';

interface CardProps extends HTMLAttributes<HTMLDivElement> {
  children: ReactNode;
}

export function Card({ children, className = '', ...props }: CardProps) {
  return (
    <div
      className={`rounded-lg border border-white/10 bg-white/5 p-6 ${className}`}
      {...props}
    >
      {children}
    </div>
  );
}

interface CardTitleProps {
  children: ReactNode;
}

export function CardTitle({ children }: CardTitleProps) {
  return (
    <h3 className="mb-2 text-lg font-semibold">
      {children}
    </h3>
  );
}

interface CardDescriptionProps {
  children: ReactNode;
}

export function CardDescription({ children }: CardDescriptionProps) {
  return (
    <p className="text-sm text-white/50">
      {children}
    </p>
  );
}
