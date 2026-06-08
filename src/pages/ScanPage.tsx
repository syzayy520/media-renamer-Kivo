import { useState } from 'react';
import { FolderOpen, Play, Settings, Clock } from 'lucide-react';
import { Button, Card, CardHeader, CardTitle, CardDescription, CardContent, CardFooter, Input, LoadingSpinner, EmptyState } from '../components/ui';
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
      // 错误已经在store中处理
      console.error('Scan failed:', err);
    }
  };

  const handleSelectFolder = async () => {
    // 这里可以集成Tauri的文件夹选择对话框
    // 暂时使用简单的输入
    const path = prompt('请输入目录路径:');
    if (path) {
      setDirectory(path);
    }
  };

  return (
    <div className="w-full px-6 py-8">
      <div className="w-full">
        <h1 className="text-3xl font-bold text-text-primary mb-8 whitespace-nowrap">
          媒体文件重命名
        </h1>

        <Card variant="elevated" className="mb-8">
          <CardHeader>
            <CardTitle>选择扫描目录</CardTitle>
            <CardDescription>
              选择要扫描的媒体文件目录，系统将自动解析文件名并生成重命名预览
            </CardDescription>
          </CardHeader>
          
          <CardContent>
            <div className="space-y-4">
              <div className="flex gap-4 items-center">
                <Input
                  placeholder="输入目录路径..."
                  value={directory}
                  onChange={(e) => setDirectory(e.target.value)}
                  className="flex-1 min-w-0"
                />
                <Button
                  variant="secondary"
                  onClick={handleSelectFolder}
                  icon={<FolderOpen className="w-4 h-4" />}
                >
                  浏览
                </Button>
              </div>
              
              {error && (
                <div className="p-3 bg-danger/10 border border-danger/30 rounded-lg">
                  <p className="text-sm text-danger">{error}</p>
                </div>
              )}
            </div>
          </CardContent>
          
          <CardFooter>
            <div className="flex justify-between items-center w-full gap-4">
              <div className="text-sm text-text-secondary min-w-0 flex-1">
                {directory ? `已选择: ${directory}` : '请选择要扫描的目录'}
              </div>
              <Button
                onClick={handleStartScan}
                disabled={!directory.trim() || isLoading}
                isLoading={isLoading}
                icon={<Play className="w-4 h-4" />}
              >
                开始扫描
              </Button>
            </div>
          </CardFooter>
        </Card>

        {/* 扫描结果预览 */}
        {pipelineResult && (
          <Card variant="elevated">
            <CardHeader>
              <CardTitle>扫描结果</CardTitle>
              <CardDescription>
                扫描完成，共找到 {pipelineResult.scan.video_count} 个视频文件
              </CardDescription>
            </CardHeader>
            
            <CardContent>
              <div className="grid grid-cols-2 md:grid-cols-4 gap-4 mb-6">
                <div className="bg-bg-secondary p-3 rounded-lg">
                  <div className="text-2xl font-bold text-accent">
                    {pipelineResult.scan.video_count}
                  </div>
                  <div className="text-sm text-text-secondary">视频文件</div>
                </div>
                <div className="bg-bg-secondary p-3 rounded-lg">
                  <div className="text-2xl font-bold text-success">
                    {pipelineResult.parsed_count}
                  </div>
                  <div className="text-sm text-text-secondary">已解析</div>
                </div>
                <div className="bg-bg-secondary p-3 rounded-lg">
                  <div className="text-2xl font-bold text-warning">
                    {pipelineResult.unknown_count}
                  </div>
                  <div className="text-sm text-text-secondary">未识别</div>
                </div>
                <div className="bg-bg-secondary p-3 rounded-lg">
                  <div className="text-2xl font-bold text-text-primary">
                    {pipelineResult.scan.scan_duration_ms}ms
                  </div>
                  <div className="text-sm text-text-secondary">扫描耗时</div>
                </div>
              </div>

              <div className="space-y-2">
                <h4 className="font-medium text-text-primary">安全检查</h4>
                <div className="grid grid-cols-2 gap-2">
                  {pipelineResult.safety.checks.map((check, index) => (
                    <div
                      key={index}
                      className={`p-2 rounded text-sm ${
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
              <div className="flex justify-end gap-4 w-full">
                <Button variant="secondary" icon={<Settings className="w-4 h-4" />}>
                  设置
                </Button>
                <Button variant="secondary" icon={<Clock className="w-4 h-4" />}>
                  历史记录
                </Button>
                <Button>
                  查看预览
                </Button>
              </div>
            </CardFooter>
          </Card>
        )}

        {/* 空状态 */}
        {!pipelineResult && !isLoading && (
          <EmptyState
            icon={<FolderOpen className="w-12 h-12" />}
            title="选择目录开始扫描"
            description="选择一个包含媒体文件的目录，系统将自动解析文件名并生成重命名预览"
          />
        )}

        {/* 加载状态 */}
        {isLoading && (
          <div className="flex justify-center py-12">
            <LoadingSpinner size="lg" text="正在扫描目录..." />
          </div>
        )}
      </div>
    </div>
  );
}
