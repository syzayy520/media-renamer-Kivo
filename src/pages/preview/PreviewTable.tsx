import { X, Edit, Search } from 'lucide-react';
import { Badge, Table, TableHeader, TableBody, TableRow, TableHead, TableCell, EmptyState, Tooltip, Button } from '../../components/ui';
import type { RenamePreviewItem } from '../../types';

interface PreviewTableProps {
  items: RenamePreviewItem[];
  selectedItems: Set<string>;
  onSelectItem: (id: string) => void;
  onSelectAll: () => void;
  filteredCount: number;
  onTmdbSearch?: (item: RenamePreviewItem) => void;
}

function getConfidenceBadge(confidence: number) {
  if (confidence >= 80) {
    return <Badge variant="success">{confidence}%</Badge>;
  } else if (confidence >= 60) {
    return <Badge variant="warning">{confidence}%</Badge>;
  } else {
    return <Badge variant="danger">{confidence}%</Badge>;
  }
}

function getStatusBadge(item: RenamePreviewItem) {
  if (item.should_skip) {
    return <Badge variant="default">跳过</Badge>;
  }
  if (item.needs_manual_review) {
    return <Badge variant="warning">需审核</Badge>;
  }
  if (item.conflicts.length > 0) {
    return <Badge variant="danger">有冲突</Badge>;
  }
  return <Badge variant="success">就绪</Badge>;
}

export function PreviewTable({ items, selectedItems, onSelectItem, onSelectAll, filteredCount, onTmdbSearch }: PreviewTableProps) {
  return (
    <Table>
      <TableHeader>
        <TableRow>
          <TableHead className="w-12">
            <input
              type="checkbox"
              checked={selectedItems.size === filteredCount && filteredCount > 0}
              onChange={onSelectAll}
              className="rounded border-text-secondary/30"
            />
          </TableHead>
          <TableHead>原始文件名</TableHead>
          <TableHead>新文件名</TableHead>
          <TableHead>类型</TableHead>
          <TableHead>置信度</TableHead>
          <TableHead>状态</TableHead>
          <TableHead className="w-24">操作</TableHead>
        </TableRow>
      </TableHeader>
      <TableBody>
        {items.length === 0 ? (
          <TableRow>
            <TableCell colSpan={7} className="text-center py-8">
              <EmptyState
                title="没有找到文件"
                description="尝试调整搜索条件或返回扫描"
              />
            </TableCell>
          </TableRow>
        ) : (
          items.map((item) => (
            <TableRow key={item.id} className={selectedItems.has(item.id) ? 'bg-accent/5' : ''}>
              <TableCell>
                <input
                  type="checkbox"
                  checked={selectedItems.has(item.id)}
                  onChange={() => onSelectItem(item.id)}
                  className="rounded border-text-secondary/30"
                />
              </TableCell>
              <TableCell>
                <div className="max-w-xs truncate font-mono text-sm">
                  {item.original_name}
                </div>
              </TableCell>
              <TableCell>
                <div className="max-w-xs truncate font-mono text-sm text-accent">
                  {item.proposed_name}
                </div>
              </TableCell>
              <TableCell>
                <Badge variant="info">{item.media_type}</Badge>
              </TableCell>
              <TableCell>
                {getConfidenceBadge(item.confidence)}
              </TableCell>
              <TableCell>
                {getStatusBadge(item)}
              </TableCell>
              <TableCell>
                <div className="flex gap-1">
                  <Tooltip content="TMDb 搜索">
                    <Button
                      variant="ghost"
                      size="sm"
                      onClick={() => onTmdbSearch?.(item)}
                    >
                      <Search className="w-4 h-4" />
                    </Button>
                  </Tooltip>
                  <Tooltip content="编辑">
                    <Button variant="ghost" size="sm">
                      <Edit className="w-4 h-4" />
                    </Button>
                  </Tooltip>
                  <Tooltip content="跳过">
                    <Button variant="ghost" size="sm">
                      <X className="w-4 h-4" />
                    </Button>
                  </Tooltip>
                </div>
              </TableCell>
            </TableRow>
          ))
        )}
      </TableBody>
    </Table>
  );
}