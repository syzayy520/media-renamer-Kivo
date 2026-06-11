import type { ReactNode } from 'react';

interface SearchTypeButtonProps {
  active: boolean;
  children: ReactNode;
  onClick: () => void;
}

export function SearchTypeButton({ active, children, onClick }: SearchTypeButtonProps) {
  return (
    <button
      className={`rounded-lg px-3 py-2 text-xs font-medium transition-colors ${active ? 'bg-primary text-bg-primary' : 'text-text-secondary hover:text-text-primary'}`}
      onClick={onClick}
    >
      {children}
    </button>
  );
}
