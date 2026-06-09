// features/preview-workbench/action-bar/DryRunActionButton.tsx — 单个禁用态按钮
// 职责：展示 disabled action 按钮，附带启用条件说明

interface DryRunActionButtonProps {
  /** 按钮文案 */
  label: string;
  /** 后续启用票号说明 */
  enabledIn: string;
}

export function DryRunActionButton({ label, enabledIn }: DryRunActionButtonProps) {
  return (
    <button
      disabled
      aria-label={`${label} — disabled until ${enabledIn}`}
      title={`${label} — enabled in ${enabledIn}`}
      className="cursor-not-allowed rounded-md bg-white/5 px-3 py-1.5 text-xs text-white/30 ring-1 ring-white/10 opacity-60"
    >
      {label}
    </button>
  );
}
