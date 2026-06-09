// features/preview-workbench/page/PreviewWorkbenchPage.tsx — Preview Workbench v2 页面骨架
// 职责：三栏工作台布局，集成 FileQueuePanel / CandidatePanel / RenamePreviewPanel

import { FileQueuePanel } from '../file-list/FileQueuePanel';
import { CandidatePanel } from '../candidate-list/CandidatePanel';
import { RenamePreviewPanel } from '../rename-preview/RenamePreviewPanel';
import { DryRunActionBar } from '../action-bar/DryRunActionBar';

export function PreviewWorkbenchPage() {
  return (
    <div className="flex h-full flex-col">
      {/* 顶部标题栏 + 实验标记 */}
      <div className="border-b border-white/10 bg-black/40 px-6 py-3">
        <div className="flex items-center gap-3">
          <h1 className="text-lg font-semold">Preview Workbench</h1>
          <span className="rounded-full bg-amber-500/20 px-2.5 py-0.5 text-xs font-medium text-amber-400">
            Experimental
          </span>
          <span className="rounded-full bg-blue-500/20 px-2.5 py-0.5 text-xs font-medium text-blue-400">
            Dry-run Only
          </span>
          <span className="rounded-full bg-gray-500/20 px-2.5 py-0.5 text-xs font-medium text-gray-400">
            No real file changes
          </span>
        </div>
        <p className="mt-1 text-xs text-white/40">
          Workbench v2 正在开发中。当前仅展示页面骨架，数据将在后续票中接入。
        </p>
      </div>

      {/* 三栏主体 */}
      <div className="flex flex-1 overflow-hidden">
        {/* 左栏：File Queue Panel */}
        <div className="w-[280px] shrink-0 border-r border-white/10 bg-black/20 p-4">
          <FileQueuePanel />
        </div>

        {/* 中栏：Candidate Panel */}
        <div className="flex-1 border-r border-white/10 bg-black/10 p-4">
          <CandidatePanel />
        </div>

        {/* 右栏：Rename Preview Panel */}
        <div className="w-[360px] shrink-0 bg-black/20 p-4">
          <RenamePreviewPanel />
        </div>
      </div>

      {/* 底部：Dry-run Action Bar */}
      <DryRunActionBar />
    </div>
  );
}
