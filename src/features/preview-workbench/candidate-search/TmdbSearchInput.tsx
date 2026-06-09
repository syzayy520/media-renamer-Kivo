// features/preview-workbench/candidate-search/TmdbSearchInput.tsx — TMDb 搜索输入框（Mock UI）
// 职责：展示搜索输入框和按钮，均为 disabled 状态
// 安全边界：不调用 backend command，不 invoke，不 fetch

/**
 * TMDb 搜索输入框 Mock UI
 * 输入框和按钮均为 disabled 状态，仅用于展示未来 UI 形态
 */
export function TmdbSearchInput() {
  return (
    <div className="flex gap-2">
      {/* 搜索输入框（disabled） */}
      <input
        type="text"
        placeholder="Search TMDb... (disabled in P2-010)"
        disabled
        className="flex-1 rounded-md border border-white/10 bg-white/5 px-3 py-2 text-sm text-white/50 placeholder:text-white/30 disabled:cursor-not-allowed disabled:opacity-50"
      />

      {/* 媒体类型选择（disabled） */}
      <select
        disabled
        className="rounded-md border border-white/10 bg-white/5 px-3 py-2 text-sm text-white/50 disabled:cursor-not-allowed disabled:opacity-50"
      >
        <option value="Movie">Movie</option>
        <option value="Tv">TV Show</option>
      </select>

      {/* 搜索按钮（disabled） */}
      <button
        disabled
        className="rounded-md bg-blue-500/20 px-4 py-2 text-sm font-medium text-blue-400/50 disabled:cursor-not-allowed disabled:opacity-50"
      >
        Search
      </button>
    </div>
  );
}
