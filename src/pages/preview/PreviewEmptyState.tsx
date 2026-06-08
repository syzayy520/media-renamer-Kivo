import { Button, EmptyState } from '../../components/ui';

export function PreviewEmptyState() {
  return (
    <EmptyState
      title="没有预览数据"
      description="请先扫描一个目录以生成重命名预览"
      action={
        <Button variant="secondary" onClick={() => window.history.back()}>
          返回扫描
        </Button>
      }
    />
  );
}