// features/scan/components/ScanActionPanel.tsx — 扫描操作面板
// 职责：Start Dry-run Scan 按钮 + loading 状态

interface ScanActionPanelProps {
  disabled: boolean;
  isScanning: boolean;
  onStart: () => void;
}

export function ScanActionPanel({
  disabled,
  isScanning,
  onStart,
}: ScanActionPanelProps) {
  return (
    <button
      type="button"
      disabled={disabled || isScanning}
      onClick={onStart}
      className={`inline-flex items-center gap-2 rounded-md px-6 py-2.5 text-sm font-medium transition-colors ${
        disabled || isScanning
          ? 'cursor-not-allowed bg-white/5 text-white/20'
          : 'bg-blue-600 text-white hover:bg-blue-500'
      }`}
    >
      {isScanning ? (
        <>
          <span className="inline-block h-4 w-4 animate-spin rounded-full border-2 border-white/30 border-t-white" aria-hidden="true" />
          扫描中...
        </>
      ) : (
        '开始 Dry-run 扫描'
      )}
    </button>
  );
}
