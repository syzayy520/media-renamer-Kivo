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
            <Globe className="h-3 w-3" />
            TMDb
          </Badge>
        </Tooltip>
      );
    case 'Manual':
      return (
        <Tooltip content="来源: 手动编辑">
          <Badge variant="warning" size="sm" className="gap-1">
            <Edit className="h-3 w-3" />
            手动
          </Badge>
        </Tooltip>
      );
    default:
      return (
        <Tooltip content="来源: 本地规则解析">
          <Badge variant="default" size="sm" className="gap-1">
            <Database className="h-3 w-3" />
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
          <div className="h-2 w-2 rounded-full bg-accent animate-pulse" />
        </Tooltip>
      );
    case 'success':
      if (itemState.results.length > 0) {
        return (
          <Tooltip content={`找到 ${itemState.results.length} 个候选`}>
            <div className="h-2 w-2 rounded-full bg-success" />
          </Tooltip>
        );
      }
      return (
        <Tooltip content="未找到匹配结果">
          <div className="h-2 w-2 rounded-full bg-text-secondary/40" />
        </Tooltip>
      );
    case 'error':
      return (
        <Tooltip content={`搜索出错: ${itemState.error ?? '未知错误'}`}>
          <div className="h-2 w-2 rounded-full bg-danger" />
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
        <SkipForward className="h-3 w-3" />
        跳过
      </Badge>
    );
  }
  if (item.needs_manual_review) {
    return (
      <Badge variant="warning" className="gap-1">
        <AlertTriangle className="h-3 w-3" />
        需审核
      </Badge>
    );
  }
  if (item.conflicts.length > 0) {
    const blocking = item.conflicts.some((conflict) => conflict.blocking);
    return (
      <Badge variant={blocking ? 'danger' : 'warning'} className="gap-1">
        <ShieldAlert className="h-3 w-3" />
        {blocking ? '阻断冲突' : '有冲突'}
      </Badge>
    );
  }
  if (item.metadata_source === 'Tmdb') {
    return (
      <Badge variant="info" className="gap-1">
        <CheckCircle2 className="h-3 w-3" />
        TMDb 已应用
      </Badge>
    );
  }
  return (
    <Badge variant="success" className="gap-1">
      <CheckCircle2 className="h-3 w-3" />
      就绪
    </Badge>
  );
}

function showPendingEditNotice() {
  window.alert('手动编辑文件名功能还未接入，后续会在预览表格内打开安全编辑面板。');
}

function showPendingSkipNotice() {
  window.alert('跳过/取消跳过功能还未接入，后续会在预览表格内直接切换。');
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
          <TableHead className="w-[260px]">原始文件名</TableHead>
          <TableHead className="w-[320px]">新文件名</TableHead>
          <TableHead className="w-[100px]">类型</TableHead>
          <TableHead className="w-[100px]">来源</TableHead>
          <TableHead className="w-[100px]">置信度</TableHead>
          <TableHead className="w-[150px]">状态</TableHead>
          <TableHead className="w-[110px] text-right">操作</TableHead>
        </TableRow>
      </TableHeader>
      <TableBody>
        {items.length === 0 ? (
          <TableRow>
            <TableCell colSpan={8} className="py-8 text-center">
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
                  <Tooltip content={item.original_name}>
                    <div className="w-full truncate font-mono text-sm leading-6 text-text-primary">
                      {item.original_name}
                    </div>
                  </Tooltip>
                </TableCell>
                <TableCell>
                  <Tooltip content={item.proposed_name}>
                    <div className="w-full truncate font-mono text-sm leading-6 text-accent">
                      {item.proposed_name}
                    </div>
                  </Tooltip>
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
                  <div className="flex justify-end gap-1">
                    {tmdbEnabled && (
                      <Tooltip content="TMDb 搜索">
                        <Button
                          variant="ghost"
                          size="sm"
                          onClick={() => onTmdbSearch?.(item)}
                        >
                          <Search className="h-4 w-4" />
                        </Button>
                      </Tooltip>
                    )}
                    <Tooltip content="编辑文件名">
                      <Button variant="ghost" size="sm" onClick={showPendingEditNotice}>
                        <Edit className="h-4 w-4" />
                      </Button>
                    </Tooltip>
                    <Tooltip content={item.should_skip ? '取消跳过' : '跳过此文件'}>
                      <Button variant="ghost" size="sm" onClick={showPendingSkipNotice}>
                        <X className="h-4 w-4" />
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
