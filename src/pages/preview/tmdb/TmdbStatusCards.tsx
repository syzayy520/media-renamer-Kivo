import { Card } from '../../../components/ui';

export function TmdbLoadingCard() {
  return (
    <Card className="flex items-center justify-center gap-2 p-4">
      <div className="h-4 w-4 animate-spin rounded-full border-2 border-primary border-t-transparent" />
      <span className="text-xs text-text-secondary">搜索中...</span>
    </Card>
  );
}

export function TmdbErrorCard({ message }: { message: string }) {
  return (
    <Card className="border-danger/30 bg-danger/10 p-3">
      <p className="text-xs text-danger">{message}</p>
    </Card>
  );
}
