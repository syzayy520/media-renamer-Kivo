import { Button, Card } from '../../../components/ui';

interface PreviewEmptyStateProps {
  onScan: () => void;
}

export function PreviewEmptyState({ onScan }: PreviewEmptyStateProps) {
  return (
    <Card className="p-6 text-center">
      <p className="mb-4 text-sm text-text-secondary">暂无预览结果，请先扫描文件</p>
      <Button onClick={onScan}>去扫描</Button>
    </Card>
  );
}
