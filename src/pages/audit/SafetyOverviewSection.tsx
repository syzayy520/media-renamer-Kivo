import { Shield, CheckCircle, FileText } from 'lucide-react';
import { Card, CardContent } from '../../components/ui';
import type { SafetyCheck } from '../../types';

interface SafetyOverviewSectionProps {
  safetyChecks: SafetyCheck[];
  canExecute: boolean;
}

export function SafetyOverviewSection({ safetyChecks, canExecute }: SafetyOverviewSectionProps) {
  const hasChecks = safetyChecks.length > 0;
  const passRate = hasChecks
    ? Math.round((safetyChecks.filter((check) => check.passed).length / safetyChecks.length) * 100)
    : 0;
  const statusLabel = !hasChecks ? '待扫描' : canExecute ? '安全' : '有风险';
  const statusClass = !hasChecks ? 'text-text-secondary' : canExecute ? 'text-success' : 'text-danger';
  const iconClass = !hasChecks ? 'text-text-secondary' : canExecute ? 'text-success' : 'text-danger';
  const iconBgClass = !hasChecks ? 'bg-text-secondary/10' : canExecute ? 'bg-success/20' : 'bg-danger/20';

  return (
    <div className="grid grid-cols-1 gap-6 md:grid-cols-3">
      <Card variant="elevated">
        <CardContent className="p-6">
          <div className="flex items-center justify-between gap-4">
            <div className="min-w-0">
              <p className="text-sm text-text-secondary">安全状态</p>
              <p className={`mt-1 text-2xl font-bold ${statusClass}`}>
                {statusLabel}
              </p>
            </div>
            <div className={`rounded-full p-3 ${iconBgClass}`}>
              <Shield className={`h-6 w-6 ${iconClass}`} />
            </div>
          </div>
        </CardContent>
      </Card>

      <Card variant="elevated">
        <CardContent className="p-6">
          <div className="flex items-center justify-between gap-4">
            <div className="min-w-0">
              <p className="text-sm text-text-secondary">检查项</p>
              <p className="mt-1 text-2xl font-bold text-text-primary">
                {safetyChecks.length}
              </p>
            </div>
            <div className="rounded-full bg-accent/20 p-3">
              <FileText className="h-6 w-6 text-accent" />
            </div>
          </div>
        </CardContent>
      </Card>

      <Card variant="elevated">
        <CardContent className="p-6">
          <div className="flex items-center justify-between gap-4">
            <div className="min-w-0">
              <p className="text-sm text-text-secondary">通过率</p>
              <p className="mt-1 text-2xl font-bold text-text-primary">
                {passRate}%
              </p>
            </div>
            <div className="rounded-full bg-success/20 p-3">
              <CheckCircle className="h-6 w-6 text-success" />
            </div>
          </div>
        </CardContent>
      </Card>
    </div>
  );
}
