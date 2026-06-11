import { Card } from '../../../components/ui';

export function TmdbEmptySelectionCard() {
  return (
    <Card className="p-4 text-center">
      <p className="text-xs text-text-tertiary">在左侧树中选择一个组查看详情</p>
    </Card>
  );
}
