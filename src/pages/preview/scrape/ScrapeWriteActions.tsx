import { useMemo, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { AlertTriangle, CheckCircle2, Download } from 'lucide-react';
import { Button } from '../../../components/ui';
import { usePipelineStore } from '../../../state/pipelineStore';
import type { FolderGroup, TmdbCandidate } from '../../../types';
import { resolveScrapeTargetFolder } from './resolveScrapeTargetFolder';

interface LocalScrapeWrittenFile {
  role: string;
  file_path: string;
}

interface LocalScrapeOutput {
  success: boolean;
  target_folder_path: string;
  written_files: LocalScrapeWrittenFile[];
  errors: string[];
}

interface ScrapeWriteActionsProps {
  candidate: TmdbCandidate;
  group: FolderGroup | null;
}

export function ScrapeWriteActions({ candidate, group }: ScrapeWriteActionsProps) {
  const scanRoot = usePipelineStore((state) => state.pipelineResult?.scan?.scan_path);
  const [isWriting, setIsWriting] = useState(false);
  const [result, setResult] = useState<LocalScrapeOutput | null>(null);
  const [error, setError] = useState<string | null>(null);

  const mainFileName = useMemo(() => {
    return group?.children.find((child) => child.file_role === 'MainVideo')?.target_name
      || group?.children[0]?.target_name
      || '';
  }, [group]);

  const targetFolder = useMemo(() => resolveScrapeTargetFolder(group, scanRoot), [group, scanRoot]);
  const canWrite = Boolean(targetFolder && mainFileName);

  const handleWrite = async () => {
    if (!group || !canWrite) {
      setError('缺少目标文件夹，无法写入刮削文件。');
      return;
    }

    setIsWriting(true);
    setError(null);
    setResult(null);

    try {
      const output = await invoke<LocalScrapeOutput>('scrape_to_local_metadata', {
        input: {
          candidate,
          target_folder_path: targetFolder,
          target_file_name: mainFileName,
        },
      });
      setResult(output);
      if (!output.success && output.errors.length > 0) {
        setError(output.errors.join('\n'));
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : String(err));
    } finally {
      setIsWriting(false);
    }
  };

  return (
    <div className="mt-3 space-y-2 border-t border-border pt-3">
      <Button
        variant="primary"
        size="sm"
        icon={<Download className="h-3.5 w-3.5" />}
        isLoading={isWriting}
        disabled={!canWrite || isWriting}
        className="w-full"
        onClick={handleWrite}
      >
        刮削到本地
      </Button>

      <div className="break-all text-xs text-text-tertiary">
        写入目录：{targetFolder || '未选择'}
      </div>

      {result && result.written_files.length > 0 && (
        <div className="rounded-lg border border-success/30 bg-success/10 p-2 text-xs text-success">
          <div className="mb-1 flex items-center gap-1 font-medium">
            <CheckCircle2 className="h-3.5 w-3.5" />
            已写入 {result.written_files.length} 个文件
          </div>
          <div className="space-y-1 text-text-secondary">
            {result.written_files.map((file) => (
              <div key={`${file.role}:${file.file_path}`} className="break-all">
                {file.role}: {file.file_path}
              </div>
            ))}
          </div>
        </div>
      )}

      {error && (
        <div className="rounded-lg border border-danger/30 bg-danger/10 p-2 text-xs text-danger">
          <div className="mb-1 flex items-center gap-1 font-medium">
            <AlertTriangle className="h-3.5 w-3.5" />
            刮削写入异常
          </div>
          <div className="whitespace-pre-wrap break-all">{error}</div>
        </div>
      )}
    </div>
  );
}
