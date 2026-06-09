// features/scan/page/ScanPage.tsx — 扫描页面
// 职责：组合扫描输入、操作、结果、安全状态、错误展示

import { useScanStore } from '../state/scanStore';
import { DirectoryInput } from '../components/DirectoryInput';
import { ScanActionPanel } from '../components/ScanActionPanel';
import { ScanResultSummary } from '../components/ScanResultSummary';
import { ScanSafetySummary } from '../components/ScanSafetySummary';
import { ScanErrorPanel } from '../components/ScanErrorPanel';

export function ScanPage() {
  const {
    directory,
    setDirectory,
    isScanning,
    error,
    result,
    setError,
    startScan,
  } = useScanStore();

  const hasResult = result !== null;
  const hasError = error !== null;

  return (
    <div className="space-y-6">
      {/* 标题 + Dry-run 安全说明 */}
      <div className="rounded-lg border border-white/10 bg-white/5 p-6">
        <h2 className="mb-2 text-lg font-semibold">媒体文件扫描</h2>
        <p className="mb-4 text-sm text-white/50">
          选择包含媒体文件的目录，系统将自动解析文件名并生成重命名预览。
        </p>
        <div className="rounded-lg border border-amber-500/20 bg-amber-500/5 px-4 py-3">
          <p className="text-sm text-amber-400">
            当前仅为 Dry-run Preview，不会真实修改媒体文件。
          </p>
        </div>
      </div>

      {/* 目录输入 + 操作按钮 */}
      <div className="rounded-lg border border-white/10 bg-white/5 p-6">
        <h3 className="mb-3 text-sm font-medium text-white/70">选择扫描目录</h3>
        <DirectoryInput value={directory} onChange={setDirectory} />
        <div className="mt-4">
          <ScanActionPanel
            disabled={!directory.trim()}
            isScanning={isScanning}
            onStart={startScan}
          />
        </div>
      </div>

      {/* 错误状态 */}
      {hasError && (
        <ScanErrorPanel message={error} onDismiss={() => setError('')} />
      )}

      {/* 扫描结果 */}
      {hasResult && (
        <>
          <div className="rounded-lg border border-white/10 bg-white/5 p-6">
            <h3 className="mb-4 text-sm font-medium text-white/70">扫描结果</h3>
            <ScanResultSummary result={result} />
          </div>

          {/* 安全检查摘要 */}
          <div>
            <ScanSafetySummary safety={result.safety} />
          </div>
        </>
      )}

      {/* 空状态 */}
      {!hasResult && !hasError && !isScanning && (
        <div className="rounded-lg border border-white/5 bg-white/[0.02] p-8 text-center">
          <p className="text-sm text-white/30">
            输入媒体文件目录路径，点击「开始 Dry-run 扫描」查看重命名预览。
          </p>
        </div>
      )}
    </div>
  );
}
