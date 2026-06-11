import { Card } from '../../../components/ui';

export function PreviewLoadingState() {
  return (
    <Card className="p-6">
      <div className="flex items-center gap-3">
        <div className="h-5 w-5 animate-spin rounded-full border-2 border-primary border-t-transparent" />
        <span className="text-sm text-text-secondary">加载中...</span>
      </div>
    </Card>
  );
}
