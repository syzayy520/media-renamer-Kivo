import { X, Edit, Search } from 'lucide-react';
import { Badge, EmptyState, Tooltip, Button } from '../../components/ui';
import type { RenamePreviewItem } from '../../types';
import type { ItemSearchState } from '../../state/tmdbSearchStore';

interface PreviewTableGridProps {
  items: RenamePreviewItem[];
  selectedItems: Set<string>;
  onSelectItem: (id: string) => void;
  onSelectAll: () => void;
  filteredCount: number;
  onTmdbSearch?: (item: RenamePreviewItem) => void;
  onManualEdit?: (item: RenamePreviewItem) => void;
  itemStates?: Record<string, ItemSearchState>;
  tmdbEnabled?: boolean;
}

function ConfidenceBadge({ confidence }: { confidence: number }) {
  if (confidence >= 80) return <Badge variant="success">{confidence}%</Badge>;
  if (confidence >= 60) return <Badge variant="warning">{confidence}%</Badge>;
  return <Badge variant="danger">{confidence}%</Badge>;
}

function SourceBadge({ source }: { source: RenamePreviewItem['metadata_source'] }) {
  if (source === 'Tmdb') return <Badge variant="info">TMDb</Badge>;
  if (source === 'Manual') return <Badge variant="warning">手动</Badge>;
  return <Badge variant="default">本地</Badge>;
}

function StatusBadge({ item }: { item: RenamePreviewItem }) {
  if (item.should_skip) return <Badge variant="default">跳过</Badge>;
  if (item.needs_manual_review) return <Badge variant="warning">需审核</Badge>;
  if (item.conflicts.length > 0) return <Badge variant="danger">阻断冲突</Badge>;
  if (item.metadata_source === 'Tmdb') return <Badge variant="info">TMDb 已应用</Badge>;
  return <Badge variant="success">就绪</Badge>;
}

function showPendingSkipNotice() {
  window.alert('跳过/取消跳过功能还未接入，后续会在预览表格内直接切换。');
}

export function PreviewTableGrid({
  items,
  selectedItems,
  onSelectItem,
  onSelectAll,
  filteredCount,
  onTmdbSearch,
  onManualEdit,
  itemStates = {},
  tmdbEnabled = false,
}: PreviewTableGridProps) {
  const allSelected = selectedItems.size === filteredCount && filteredCount > 0;

  if (items.length === 0) {
    return (
      <div className="py-10">
        <EmptyState title="没有找到文件" description="尝试调整搜索条件或返回扫描" />
      </div>
    );
  }

  return (
    <div className="w-full overflow-x-auto">
      <div className="min-w-[1120px]">
        <div className="grid grid-cols-[44px_minmax(260px,1.2fr)_minmax(340px,1.4fr)_92px_92px_92px_138px_104px] items-center border-b border-text-secondary/20 bg-bg-secondary px-4 py-3 text-xs font-medium uppercase text-text-secondary">
          <input type="checkbox" checked={allSelected} onChange={onSelectAll} />
          <div>原始文件名</div>
          <div>新文件名</div>
          <div>类型</div>
          <div>来源</div>
          <div>置信度</div>
          <div>状态</div>
          <div className="text-right">操作</div>
        </div>

        {items.map((item) => {
          const selected = selectedItems.has(item.id);
          const itemState = itemStates[item.id];

          return (
            <div
              key={item.id}
              className={`grid grid-cols-[44px_minmax(260px,1.2fr)_minmax(340px,1.4fr)_92px_92px_92px_138px_104px] items-center border-b border-text-secondary/20 px-4 py-3 hover:bg-bg-secondary/50 ${selected ? 'bg-accent/5' : ''}`}
            >
              <input type="checkbox" checked={selected} onChange={() => onSelectItem(item.id)} />
              <Tooltip content={item.original_name}>
                <div className="min-w-0 truncate pr-4 font-mono text-sm leading-6 text-text-primary">
                  {item.original_name}
                </div>
              </Tooltip>
              <Tooltip content={item.proposed_name}>
                <div className="min-w-0 truncate pr-4 font-mono text-sm leading-6 text-accent">
                  {item.proposed_name}
                </div>
              </Tooltip>
              <Badge variant="info">{item.media_type}</Badge>
              <SourceBadge source={item.metadata_source} />
              <ConfidenceBadge confidence={item.confidence} />
              <div className="flex items-center gap-1.5">
                <StatusBadge item={item} />
                {tmdbEnabled && itemState?.status === 'loading' && <span className="h-2 w-2 rounded-full bg-accent animate-pulse" />}
              </div>
              <div className="flex justify-end gap-1">
                {tmdbEnabled && (
                  <Tooltip content="TMDb 搜索">
                    <Button variant="ghost" size="sm" onClick={() => onTmdbSearch?.(item)}>
                      <Search className="h-4 w-4" />
                    </Button>
                  </Tooltip>
                )}
                <Tooltip content="编辑文件名">
                  <Button variant="ghost" size="sm" onClick={() => onManualEdit?.(item)}>
                    <Edit className="h-4 w-4" />
                  </Button>
                </Tooltip>
                <Tooltip content={item.should_skip ? '取消跳过' : '跳过此文件'}>
                  <Button variant="ghost" size="sm" onClick={showPendingSkipNotice}>
                    <X className="h-4 w-4" />
                  </Button>
                </Tooltip>
              </div>
            </div>
          );
        })}
      </div>
    </div>
  );
}
