import { type HTMLAttributes } from 'react';
import { Loader2 } from 'lucide-react';

interface LoadingSpinnerProps extends HTMLAttributes<HTMLDivElement> {
  size?: 'sm' | 'md' | 'lg';
  text?: string;
}

export function LoadingSpinner({
  size = 'md',
  text,
  className = '',
  ...props
}: LoadingSpinnerProps) {
  const sizeClasses = {
    sm: 'w-4 h-4',
    md: 'w-8 h-8',
    lg: 'w-12 h-12',
  };

  return (
    <div
      className={`flex flex-col items-center justify-center ${className}`}
      {...props}
    >
      <Loader2 className={`${sizeClasses[size]} text-accent animate-spin`} />
      {text && (
        <p className="mt-2 text-sm text-text-secondary">{text}</p>
      )}
    </div>
  );
}
