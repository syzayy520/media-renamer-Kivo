import { useState } from 'react';
import { FolderOpen, Play, Settings, Clock } from 'lucide-react';
import { Button, Card, CardHeader, CardTitle, CardDescription, CardContent, CardFooter, Input, LoadingSpinner } from '../components/ui';
import { usePipelineStore } from '../state/pipelineStore';
import { useUiFeedbackStore } from '../state/uiFeedbackStore';

export function ScanPage() {
  const [directory, setDirectory] = useState('');
  const { startRenameSession, pipelineResult } = usePipelineStore();
  const { isLoading, error } = useUiFeedbackStore();

  const handleStartScan = async () => {
    if (!directory.trim()) return;
    
    try {
      await startRenameSession(directory);
    } catch (err) {
      console.error('Scan failed:', err);
    }
  };

  const handleSelectFolder = async () => {
    const path = prompt('请输入目录路径:');
    if (path) {
      setDirectory(path);
    }
  };

  return (
    <div className="w-full px-6 py-8">
      <div className="mx-auto w-full max-w-6xl space-y-8">
        <div className="space-y-2">
          <h1 className="text-3xl font-bold leading-tight text-text-primary">
            媒体文件重命名
          </h1>
          <p className="w-[760px] max-w-full whitespace-normal text-sm leading-6 text-text-secondary">
            选择本地媒体目录，生成重命名预览，再进入安全检查和执行流程。
          </p>
        </div>

        <Card variant="elevated" padding="lg">
          <div className="grid gap-6 lg:grid-cols-[1fr_auto] lg:items-end">
            <div className="min-w-0">
              <CardHeader>
                <CardTitle>选择扫描目录</CardTitle>
                <CardDescription>
                  系统会解析目录内的媒体文件名，并生成可审核的重命名预览。
                </CardDescription>
              </CardHeader>

              <CardContent>
                <div className="flex min-w-0 flex-col gap-3 sm:flex-row sm:items-center">
                  <Input
                    placeholder="输入目录路径..."
                    value={directory}
                    onChange={(event) => setDirectory(event.target.value)}
                    className="font-path"
                  />
                  <Button
                    variant="secondary"
                    onClick={handleSelectFolder}
                    icon={<FolderOpen className="h-4 w-4" />}
                  >
                    浏览
                  </Button>
                </div>

                {error && (
                  <div className="mt-4 rounded-xl border border-danger/30 bg-danger/10 p-3">
                    <p className="text-sm leading-6 text-danger">{error}</p>
                  </div>
                )}
              </CardContent>
            </div>

            <div className="flex justify-start lg:justify-end">
              <Button
                onClick={handleStartScan}
                disabled={!directory.trim() || isLoading}
                isLoading={isLoading}
                icon={<Play className="h-4 w-4" />}
              >
                开始扫描
              </Button>
            </div>
          </div>

          <CardFooter>
            <div className="flex min-w-0 items-center gap-2 text-sm text-text-secondary">
              <FolderOpen className="h-4 w-4 shrink-0" />
              <span className="min-w-0 truncate">
                {directory ? `已选择：${directory}` : '请选择要扫描的目录'}
              </span>
            </div>
          </CardFooter>
        </Card>

        {pipelineResult && (
          <Card variant="elevated" padding="lg">
            <CardHeader>
              <CardTitle>扫描结果</CardTitle>
              <CardDescription>
                扫描完成，共找到 {pipelineResult.scan.video_count} 个视频文件。
              </CardDescription>
            </CardHeader>
            
            <CardContent>
              <div className="mb-6 grid grid-cols-2 gap-4 md:grid-cols-4">
                <div className="rounded-xl bg-bg-secondary p-4">
                  <div className="text-2xl font-bold text-accent">
                    {pipelineResult.scan.video_count}
                  </div>
                  <div className="mt-1 text-sm text-text-secondary">视频文件</div>
                </div>
                <div className="rounded-xl bg-bg-secondary p-4">
                  <div className="text-2xl font-bold text-success">
                    {pipelineResult.parsed_count}
                  </div>
                  <div className="mt-1 text-sm text-text-secondary">已解析</div>
                </div>
                <div className="rounded-xl bg-bg-secondary p-4">
                  <div className="text-2xl font-bold text-warning">
                    {pipelineResult.unknown_count}
                  </div>
                  <div className="mt-1 text-sm text-text-secondary">未识别</div>
                </div>
                <div className="rounded-xl bg-bg-secondary p-4">
                  <div className="text-2xl font-bold text-text-primary">
                    {pipelineResult.scan.scan_duration_ms}ms
                  </div>
                  <div className="mt-1 text-sm text-text-secondary">扫描耗时</div>
                </div>
              </div>

              <div className="space-y-3">
                <h4 className="font-medium text-text-primary">安全检查</h4>
                <div className="grid gap-2 md:grid-cols-2">
                  {pipelineResult.safety.checks.map((check, index) => (
                    <div
                      key={index}
                      className={`rounded-xl p-3 text-sm ${
                        check.passed
                          ? 'bg-success/10 text-success'
                          : 'bg-danger/10 text-danger'
                      }`}
                    >
                      {check.passed ? '✓' : '✗'} {check.name}
                    </div>
                  ))}
                </div>
              </div>
            </CardContent>
            
            <CardFooter>
              <div className="flex flex-wrap justify-end gap-3">
                <Button variant="secondary" icon={<Settings className="h-4 w-4" />}>
                  设置
                </Button>
                <Button variant="secondary" icon={<Clock className="h-4 w-4" />}>
                  历史记录
                </Button>
                <Button>
                  查看预览
                </Button>
              </div>
            </CardFooter>
          </Card>
        )}

        {!pipelineResult && !isLoading && (
          <Card variant="outlined" padding="lg">
            <div className="flex flex-col gap-2 text-sm leading-6 text-text-secondary">
              <span className="font-medium text-text-primary">下一步</span>
              <span>扫描完成后，会自动进入预览、安全检查、执行确认和审计记录流程。</span>
            </div>
          </Card>
        )}

        {isLoading && (
          <Card variant="outlined" padding="lg">
            <LoadingSpinner size="lg" text="正在扫描目录..." />
          </Card>
        )}
      </div>
    </div>
  );
}
