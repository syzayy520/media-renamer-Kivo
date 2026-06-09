// features/preview/components/PreviewToolbar.tsx — 工具栏
// 职责：搜索框 + 筛选按钮

interface PreviewToolbarProps {
  searchText: string;
  onSearchChange: (text: string) => void;
  activeFilter: 'all' | 'conflict' | 'needs_review' | 'safe';
  onFilterChange: (filter: 'all' | 'conflict' | 'needs_review' | 'safe') => void;
  totalCount: number;
  filteredCount: number;
}

const filters: { key: 'all' | 'conflict' | 'needs_review' | 'safe'; label: string }[] = [
  { key: 'all', label: '全部' },
  { key: 'conflict', label: '有冲突' },
  { key: 'needs_review', label: '需审核' },
  { key: 'safe', label: '安全' },
];

export function PreviewToolbar({
  searchText,
  onSearchChange,
  activeFilter,
  onFilterChange,
  totalCount,
  filteredCount,
}: PreviewToolbarProps) {
  return (
    <div className="flex items-center justify-between">
      <div className="flex items-center gap-3">
        <input
          type="text"
          aria-label="搜索文件名"
          value={searchText}
          onChange={(e) => onSearchChange(e.target.value)}
          placeholder="搜索原文件名或新文件名..."
          className="w-64 rounded-md border border-white/10 bg-white/5 px-3 py-1.5 text-sm text-white placeholder-white/30 outline-none transition-colors focus:border-blue-500/50"
        />
        <span className="text-xs text-white/30">
          {filteredCount !== totalCount ? `${filteredCount} / ${totalCount}` : `${totalCount}`} 项
        </span>
      </div>
      <div className="flex gap-1">
        {filters.map((f) => (
          <button
            key={f.key}
            type="button"
            onClick={() => onFilterChange(f.key)}
            className={`rounded px-3 py-1 text-xs transition-colors ${
              activeFilter === f.key
                ? 'bg-blue-600/30 text-blue-400'
                : 'text-white/40 hover:bg-white/5 hover:text-white/60'
            }`}
          >
            {f.label}
          </button>
        ))}
      </div>
    </div>
  );
}
