// pages/preview/PreviewTable 组件
// 职责：渲染重命名预览表格，展示每个文件的状态和操作

import { X, Edit, Search, Database, Globe, AlertTriangle, CheckCircle2, SkipForward, ShieldAlert } from 'lucide-react';
import { Badge, Table, TableHeader, TableBody, TableRow, TableHead, TableCell, EmptyState, Tooltip, Button } from '../../components/ui';
import type { RenamePreviewItem, MetadataSource } from '../../types';
import type { ItemSearchState } from '../../state/tmdbSearchStore';

interface PreviewTableProps {
  items: RenamePreviewItem[];
  selectedItems: Set<string>;
  onSelectItem: (id: string) => void;
  onSelectAll: () => void;
  filteredCount: number;
  onTmdbSearch?: (item: RenamePreviewItem) => void;
  itemStates?: Record<string, ItemSearchState>;
  tmdbEnabled?: boolean;
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

function getMetadataSourceBadge(source: MetadataSource) {
  switch (source) {
    case 'Tmdb':
      return (
        <Tooltip content="来源: TMDb 在线搜索">
          <Badge variant="info" size="sm" className="gap-1">
            <Globe className="w-3 h-3" />
            TMDb
          </Badge>
        </Tooltip>
      );
    case 'Manual':
      return (
        <Tooltip content="来源: 手动编辑">
          <Badge variant="warning" size="sm" className="gap-1">
            <Edit className="w-3 h-3" />
            手动
          </Badge>
        </Tooltip>
      );
    default:
      return (
        <Tooltip content="来源: 本地规则解析">
          <Badge variant="default" size="sm" className="gap-1">
            <Database className="w-3 h-3" />
            本地
          </Badge>
        </Tooltip>
      );
  }
}

function getTmdbStatusIndicator(itemState: ItemSearchState | undefined) {
  if (!itemState) return null;

  switch (itemState.status) {
    case 'loading':
      return (
        <Tooltip content="正在搜索 TMDb...">
          <div className="w-2 h-2 rounded-full bg-accent animate-pulse" />
        </Tooltip>
      );
    case 'success':
      if (itemState.results.length > 0) {
        return (
          <Tooltip content={`找到 ${itemState.results.length} 个候选`}>
            <div className="w-2 h-2 rounded-full bg-success" />
          </Tooltip>
        );
      }
      return (
        <Tooltip content="未找到匹配结果">
          <div className="w-2 h-2 rounded-full bg-text-secondary/40" />
        </Tooltip>
      );
    case 'error':
      return (
        <Tooltip content={`搜索出错: ${itemState.error ?? '未知错误'}`}>
          <div className="w-2 h-2 rounded-full bg-error" />
        </Tooltip>
      );
    default:
      return null;
  }
}

function getStatusBadge(item: RenamePreviewItem) {
  if (item.should_skip) {
    return (
      <Badge variant="default" className="gap-1">
        <SkipForward className="w-3 h-3" />
        跳过
      </Badge>
    );
  }
  if (item.needs_manual_review) {
    return (
      <Badge variant="warning" className="gap-1">
        <AlertTriangle className="w-3 h-3" />
        需审核
      </Badge>
    );
  }
  if (item.conflicts.length > 0) {
    const blocking = item.conflicts.some((c) => c.blocking);
    return (
      <Badge variant={blocking ? 'danger' : 'warning'} className="gap-1">
        <ShieldAlert className="w-3 h-3" />
        {blocking ? '阻断冲突' : '有冲突'}
      </Badge>
    );
  }
  if (item.metadata_source === 'Tmdb') {
    return (
      <Badge variant="info" className="gap-1">
        <CheckCircle2 className="w-3 h-3" />
        TMDb 已应用
      </Badge>
    );
  }
  return (
    <Badge variant="success" className="gap-1">
      <CheckCircle2 className="w-3 h-3" />
      就绪
    </Badge>
  );
}

export function PreviewTable({
  items,
  selectedItems,
  onSelectItem,
  onSelectAll,
  filteredCount,
  onTmdbSearch,
  itemStates = {},
  tmdbEnabled = false,
}: PreviewTableProps) {
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
          <TableHead>来源</TableHead>
          <TableHead>置信度</TableHead>
          <TableHead>状态</TableHead>
          <TableHead className="w-24">操作</TableHead>
        </TableRow>
      </TableHeader>
      <TableBody>
        {items.length === 0 ? (
          <TableRow>
            <TableCell colSpan={8} className="text-center py-8">
              <EmptyState
                title="没有找到文件"
                description="尝试调整搜索条件或返回扫描"
              />
            </TableCell>
          </TableRow>
        ) : (
          items.map((item) => {
            const itemState = itemStates[item.id];
            return (
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
                  {getMetadataSourceBadge(item.metadata_source)}
                </TableCell>
                <TableCell>
                  {getConfidenceBadge(item.confidence)}
                </TableCell>
                <TableCell>
                  <div className="flex items-center gap-1.5">
                    {getStatusBadge(item)}
                    {tmdbEnabled && getTmdbStatusIndicator(itemState)}
                  </div>
                </TableCell>
                <TableCell>
                  <div className="flex gap-1">
                    {tmdbEnabled && (
                      <Tooltip content="TMDb 搜索">
                        <Button
                          variant="ghost"
                          size="sm"
                          onClick={() => onTmdbSearch?.(item)}
                        >
                          <Search className="w-4 h-4" />
                        </Button>
                      </Tooltip>
                    )}
                    <Tooltip content="编辑文件名">
                      <Button variant="ghost" size="sm">
                        <Edit className="w-4 h-4" />
                      </Button>
                    </Tooltip>
                    <Tooltip content={item.should_skip ? '取消跳过' : '跳过此文件'}>
                      <Button variant="ghost" size="sm">
                        <X className="w-4 h-4" />
                      </Button>
                    </Tooltip>
                  </div>
                </TableCell>
              </TableRow>
            );
          })
        )}
      </TableBody>
    </Table>
  );
}
