import { ArrowLeft, Play, Settings } from 'lucide-react';
import { Badge, Button } from '../../../components/ui';

interface PreviewHeaderProps {
  groupCount: number;
  fileCount: number;
  onBack: () => void;
  onSettings: () => void;
  onDryRun: () => void;
  onExecute: () => void;
}

export function PreviewHeader({
  groupCount,
  fileCount,
  onBack,
  onSettings,
  onDryRun,
  onExecute,
}: PreviewHeaderProps) {
  return (
    <div className="flex shrink-0 items-center justify-between">
      <div className="flex items-center gap-3">
        <Button variant="ghost" size="sm" icon={<ArrowLeft className="h-4 w-4" />} onClick={onBack}>返回</Button>
        <h2 className="text-lg font-semibold text-text-primary">预览重命名</h2>
        <Badge variant="default" size="sm">{groupCount} 组 · {fileCount} 文件</Badge>
      </div>
      <div className="flex items-center gap-2">
        <Button variant="secondary" size="sm" icon={<Settings className="h-4 w-4" />} onClick={onSettings}>设置</Button>
        <Button variant="secondary" size="sm" icon={<Play className="h-4 w-4" />} onClick={onDryRun}>模拟执行</Button>
        <Button variant="primary" size="sm" icon={<Play className="h-4 w-4" />} onClick={onExecute}>执行重命名</Button>
      </div>
    </div>
  );
}
