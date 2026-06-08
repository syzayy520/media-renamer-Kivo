// features/preview/components/PreviewStatusBadge.tsx — 状态标签
// 职责：根据 preview item 状态显示对应 badge

import type { RenamePreviewItem } from '../../../api/session/types';
import { Badge } from '../../../shared/ui/Badge';

interface PreviewStatusBadgeProps {
  item: RenamePreviewItem;
}

export function PreviewStatusBadge({ item }: PreviewStatusBadgeProps) {
  if (item.conflicts.length > 0) {
    return <Badge variant="danger">有冲突 ({item.conflicts.length})</Badge>;
  }
  if (item.needs_manual_review) {
    return <Badge variant="warning">需审核</Badge>;
  }
  return <Badge variant="success">安全</Badge>;
}
