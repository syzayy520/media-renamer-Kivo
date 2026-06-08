import { Check } from 'lucide-react';
import { Button } from '../../components/ui';

interface PreviewActionBarProps {
  selectedCount: number;
  onClearSelection: () => void;
}

export function PreviewActionBar({ selectedCount, onClearSelection }: PreviewActionBarProps) {
  if (selectedCount === 0) return null;

  return (
    <div className="fixed bottom-0 left-0 right-0 bg-bg-card border-t border-text-secondary/20 p-4">
      <div className="max-w-6xl mx-auto flex justify-between items-center">
        <div className="text-text-primary">
          已选择 <span className="font-bold text-accent">{selectedCount}</span> 个文件
        </div>
        <div className="flex gap-4">
          <Button variant="secondary" onClick={onClearSelection}>
            清除选择
          </Button>
          <Button icon={<Check className="w-4 h-4" />}>
            执行重命名
          </Button>
        </div>
      </div>
    </div>
  );
}