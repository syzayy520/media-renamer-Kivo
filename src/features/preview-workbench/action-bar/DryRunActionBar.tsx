// features/preview-workbench/action-bar/DryRunActionBar.tsx — Dry-run Action Bar 功能族
// 职责：底部 action bar 展示骨架，全部按钮 disabled，不调用后端
// 禁止：invoke / execute_rename / rollback / 真实导出

import { DryRunActionButton } from './DryRunActionButton';
import { DryRunActionStatus } from './DryRunActionStatus';

/** 本票固定的 disabled action 列表 */
const DRY_RUN_ACTIONS = [
  { label: 'Apply Candidate', enabledIn: 'P2-007' },
  { label: 'Clear Selection', enabledIn: 'P2-008' },
  { label: 'Export Plan', enabledIn: 'P2-009' },
] as const;

export function DryRunActionBar() {
  return (
    <div className="flex h-[56px] shrink-0 items-center justify-between border-t border-white/10 bg-black/40 px-6">
      <DryRunActionStatus />
      <div className="flex gap-2">
        {DRY_RUN_ACTIONS.map((action) => (
          <DryRunActionButton
            key={action.label}
            label={action.label}
            enabledIn={action.enabledIn}
          />
        ))}
      </div>
    </div>
  );
}
