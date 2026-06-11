import { Card } from '../../../components/ui';
import { PreviewPlanTree, type PreviewPlanTreeProps } from './PreviewPlanTree';

export function PreviewPlanTreePanel(props: PreviewPlanTreeProps) {
  return (
    <div className="flex min-w-0 flex-1 flex-col">
      <Card className="min-h-0 flex-1 overflow-y-auto p-0">
        <PreviewPlanTree {...props} />
      </Card>
    </div>
  );
}
