// features/preview-workbench/candidate-list/CandidateEmptyState.tsx — 候选空状态
// 职责：无候选匹配时的提示

export function CandidateEmptyState() {
  return (
    <div className="mt-4 rounded-lg border border-white/10 bg-white/5 p-4 text-center">
      <div className="mb-2 text-sm text-white/40">暂无候选匹配</div>
      <p className="text-xs text-white/30">
        候选匹配将在后续票（P2-004）接入本地候选 / 未来 TMDb 搜索
      </p>
      <p className="mt-2 text-xs text-white/20">
        当前为静态骨架，不调用真实数据
      </p>
    </div>
  );
}
