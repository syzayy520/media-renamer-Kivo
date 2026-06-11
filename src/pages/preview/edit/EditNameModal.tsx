import type { KeyboardEvent } from 'react';
import { useCallback, useState } from 'react';
import { Check, Edit3, X } from 'lucide-react';
import { Button } from '../../../components/ui';

interface EditNameModalProps {
  mode: 'file' | 'group';
  currentName: string;
  previewPath: string;
  onConfirm: (newName: string) => void;
  onCancel: () => void;
}

export function EditNameModal({
  mode,
  currentName,
  previewPath,
  onConfirm,
  onCancel,
}: EditNameModalProps) {
  const [value, setValue] = useState(currentName);

  const handleSubmit = useCallback(() => {
    const trimmed = value.trim();
    if (trimmed && trimmed !== currentName) onConfirm(trimmed);
    else onCancel();
  }, [value, currentName, onConfirm, onCancel]);

  const handleKeyDown = useCallback((event: KeyboardEvent) => {
    if (event.key === 'Enter') handleSubmit();
    if (event.key === 'Escape') onCancel();
  }, [handleSubmit, onCancel]);

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/70" onClick={onCancel}>
      <div className="w-[520px] max-w-[95vw] overflow-hidden rounded-2xl border border-accent/40 bg-bg-secondary p-0 shadow-2xl" onClick={(event) => event.stopPropagation()}>
        <div className="flex items-center justify-between border-b border-accent/20 bg-bg-card px-5 py-4">
          <div className="flex items-center gap-2">
            <Edit3 className="h-4 w-4 text-accent" />
            <h3 className="text-sm font-semibold text-text-primary">{mode === 'group' ? '编辑文件夹名' : '编辑文件名'}</h3>
          </div>
          <Button variant="ghost" size="sm" icon={<X className="h-4 w-4" />} onClick={onCancel}>{''}</Button>
        </div>
        <div className="space-y-4 p-5">
          <div>
            <label className="mb-1 block text-xs text-text-secondary">原名称</label>
            <div className="break-all rounded-xl border border-text-secondary/20 bg-bg-primary px-3 py-2 text-sm text-text-primary">{currentName}</div>
          </div>
          <div>
            <label className="mb-1 block text-xs text-text-secondary">新名称</label>
            <input
              type="text"
              className="w-full rounded-xl border border-accent/50 bg-bg-primary px-3 py-2 text-sm text-text-primary transition-colors focus:border-accent focus:outline-none"
              value={value}
              onChange={(event) => setValue(event.target.value)}
              onKeyDown={handleKeyDown}
              autoFocus
            />
          </div>
          <div>
            <label className="mb-1 block text-xs text-text-secondary">最终路径预览</label>
            <div className="break-all rounded-xl border border-text-secondary/20 bg-bg-primary px-3 py-2 font-mono text-xs text-text-secondary">
              {previewPath.replace(currentName, value.trim() || currentName)}
            </div>
          </div>
        </div>
        <div className="flex justify-end gap-2 border-t border-accent/20 bg-bg-card px-5 py-4">
          <Button variant="secondary" size="sm" onClick={onCancel}>取消</Button>
          <Button variant="primary" size="sm" icon={<Check className="h-4 w-4" />} onClick={handleSubmit}>保存</Button>
        </div>
      </div>
    </div>
  );
}
