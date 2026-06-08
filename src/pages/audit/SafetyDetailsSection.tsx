import { AlertTriangle, CheckCircle, XCircle, Shield } from 'lucide-react';
import { Button, Card, CardHeader, CardTitle, CardDescription, CardContent, CardFooter, Badge, EmptyState } from '../../components/ui';
import type { SafetyCheck } from '../../types';

interface SafetyDetailsSectionProps {
  safetyChecks: SafetyCheck[];
  canExecute: boolean;
}

export function SafetyDetailsSection({ safetyChecks, canExecute }: SafetyDetailsSectionProps) {
  return (
    <Card variant="elevated" className="mb-8">
      <CardHeader>
        <CardTitle>安全检查详情</CardTitle>
        <CardDescription>
          所有检查项必须通过才能执行重命名操作
        </CardDescription>
      </CardHeader>
      <CardContent>
        {safetyChecks.length === 0 ? (
          <EmptyState
            icon={<Shield className="w-12 h-12" />}
            title="暂无安全检查数据"
            description="请先扫描目录以生成安全检查报告"
          />
        ) : (
          <div className="space-y-4">
            {safetyChecks.map((check, index) => (
              <div
                key={index}
                className={`p-4 rounded-lg border ${
                  check.passed 
                    ? 'border-success/30 bg-success/5' 
                    : 'border-danger/30 bg-danger/5'
                }`}
              >
                <div className="flex items-center justify-between">
                  <div className="flex items-center gap-3">
                    {check.passed ? (
                      <CheckCircle className="w-5 h-5 text-success" />
                    ) : (
                      <XCircle className="w-5 h-5 text-danger" />
                    )}
                    <div>
                      <h4 className="font-medium text-text-primary capitalize">
                        {check.name.replace('_', ' ')}
                      </h4>
                      <p className="text-sm text-text-secondary">
                        {check.message}
                      </p>
                    </div>
                  </div>
                  <Badge variant={check.passed ? 'success' : 'danger'}>
                    {check.passed ? '通过' : '失败'}
                  </Badge>
                </div>
              </div>
            ))}
          </div>
        )}
      </CardContent>
      {safetyChecks.length > 0 && (
        <CardFooter>
          <div className="flex justify-between items-center w-full">
            <div className="text-sm text-text-secondary">
              {canExecute ? '所有检查通过，可以安全执行' : '存在阻止执行的问题，请先解决'}
            </div>
            <Button
              disabled={!canExecute}
              icon={<AlertTriangle className="w-4 h-4" />}
            >
              {canExecute ? '继续执行' : '无法执行'}
            </Button>
          </div>
        </CardFooter>
      )}
    </Card>
  );
}