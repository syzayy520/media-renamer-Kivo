import { useEffect, useState } from 'react';
import { Button, Input } from '../../components/ui';
import type { RenamePreviewItem } from '../../types';

interface PreviewNameEditDialogProps {
  item: RenamePreviewItem | null;
  onSave: (itemId: string, proposedName: string) => void;
  onClose: () => void;
}

export function PreviewNameEditDialog({ item, onSave, onClose }: PreviewNameEditDialogProps) {
  const [proposedName, setProposedName] = useState('');

  useEffect(() => {
    setProposedName(item?.proposed_name ?? '');
  }, [item]);

  if (!item) {
    return null;
  }

  const trimmedName = proposedName.trim();
  const canSave = trimmedName.length > 0 && trimmedName !== item.proposed_name;

  const handleSave = () => {
    if (canSave) {
      onSave(item.id, trimmedName);
    }
  };

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-bg-primary/80 px-6">
      <div className="w-full max-w-2xl rounded-2xl border border-text-secondary/20 bg-bg-secondary p-6">
        <div className="space-y-1">
          <h2 className="text-xl font-semibold text-text-primary">编辑预览文件名</h2>
          <p className="text-sm leading-6 text-text-secondary">
            只修改当前预览项，执行前仍会经过安全检查。
          </p>
        </div>

        <div className="mt-5 space-y-4">
          <div>
            <p className="mb-1 text-sm font-medium text-text-primary">原始文件名</p>
            <div className="rounded-xl bg-bg-primary px-4 py-3 font-mono text-sm leading-6 text-text-secondary">
              {item.original_name}
            </div>
          </div>

          <Input
            label="新文件名"
            value={proposedName}
            onChange={(event) => setProposedName(event.target.value)}
            className="font-path"
            autoFocus
          />
        </div>

        <div className="mt-6 flex justify-end gap-3">
          <Button variant="secondary" onClick={onClose}>
            取消
          </Button>
          <Button onClick={handleSave} disabled={!canSave}>
            保存修改
          </Button>
        </div>
      </div>
    </div>
  );
}
