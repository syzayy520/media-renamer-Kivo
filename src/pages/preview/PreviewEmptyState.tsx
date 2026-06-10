import { Eye } from 'lucide-react';
import { useNavigate } from 'react-router-dom';
import { Button, Card, EmptyState } from '../../components/ui';

export function PreviewEmptyState() {
  const navigate = useNavigate();

  return (
    <div className="w-full px-6 py-8">
      <div className="mx-auto flex min-h-[520px] w-full max-w-6xl items-center justify-center">
        <Card variant="outlined" padding="lg" className="max-w-3xl">
          <EmptyState
            icon={<Eye className="h-10 w-10" />}
            title="暂无预览数据"
            description="请先扫描一个目录，生成重命名预览后再进入审核和执行流程。"
            action={
              <Button variant="secondary" onClick={() => navigate('/')}>
                返回扫描
              </Button>
            }
          />
        </Card>
      </div>
    </div>
  );
}
