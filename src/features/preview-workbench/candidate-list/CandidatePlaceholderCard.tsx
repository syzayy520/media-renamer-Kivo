// features/preview-workbench/candidate-list/CandidatePlaceholderCard.tsx — 候选占位卡片
// 职责：静态占位卡片，模拟未来候选列表项

interface CandidatePlaceholderCardProps {
  index: number;
}

export function CandidatePlaceholderCard({ index }: CandidatePlaceholderCardProps) {
  return (
    <div className="mb-2 rounded-lg border border-white/10 bg-white/5 p-3 opacity-40">
      {/* 候选标题占位 */}
      <div className="mb-2 h-4 w-3/4 rounded bg-white/10" />

      {/* 评分占位 */}
      <div className="mb-2 flex gap-2">
        <div className="h-3 w-16 rounded bg-white/10" />
        <div className="h-3 w-20 rounded bg-white/10" />
      </div>

      {/* 年份/类型占位 */}
      <div className="flex gap-2 text-xs text-white/30">
        <div className="h-3 w-12 rounded bg-white/10" />
        <div className="h-3 w-24 rounded bg-white/10" />
      </div>

      {/* 占位标记 */}
      <div className="mt-2 text-center text-xs text-white/20">
        占位卡片 {index + 1}
      </div>
    </div>
  );
}
