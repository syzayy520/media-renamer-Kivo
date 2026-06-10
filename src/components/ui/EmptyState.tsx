import { type HTMLAttributes, type ReactNode } from 'react';

interface EmptyStateProps extends HTMLAttributes<HTMLDivElement> {
  icon?: ReactNode;
  title: string;
  description?: string;
  action?: ReactNode;
}

export function EmptyState({
  icon,
  title,
  description,
  action,
  className = '',
  ...props
}: EmptyStateProps) {
  return (
    <div
      className={`w-full min-w-0 flex flex-col items-center justify-center px-6 py-12 text-center ${className}`}
      {...props}
    >
      {icon && (
        <div className="mb-5 flex h-16 w-16 items-center justify-center rounded-2xl border border-text-secondary/20 bg-bg-secondary/70 text-text-secondary">
          {icon}
        </div>
      )}
      <h3 className="mb-2 max-w-full text-lg font-semibold leading-7 text-text-primary whitespace-nowrap">
        {title}
      </h3>
      {description && (
        <p className="mb-5 w-full max-w-xl text-sm leading-6 text-text-secondary break-keep">
          {description}
        </p>
      )}
      {action && (
        <div className="inline-flex items-center justify-center">
          {action}
        </div>
      )}
    </div>
  );
}
