import { Shield, CheckCircle, FileText } from 'lucide-react';
import { Card, CardContent } from '../../components/ui';
import type { SafetyCheck } from '../../types';

interface SafetyOverviewSectionProps {
  safetyChecks: SafetyCheck[];
  canExecute: boolean;
}

export function SafetyOverviewSection({ safetyChecks, canExecute }: SafetyOverviewSectionProps) {
  const passRate = safetyChecks.length > 0 
    ? Math.round((safetyChecks.filter(c => c.passed).length / safetyChecks.length) * 100)
    : 0;

  return (
    <div className="grid grid-cols-1 md:grid-cols-3 gap-6 mb-8">
      <Card variant="elevated">
        <CardContent className="p-6">
          <div className="flex items-center justify-between">
            <div>
              <p className="text-sm text-text-secondary">安全状态</p>
              <p className={`text-2xl font-bold ${canExecute ? 'text-success' : 'text-danger'}`}>
                {canExecute ? '安全' : '有风险'}
              </p>
            </div>
            <div className={`p-3 rounded-full ${canExecute ? 'bg-success/20' : 'bg-danger/20'}`}>
              <Shield className={`w-6 h-6 ${canExecute ? 'text-success' : 'text-danger'}`} />
            </div>
          </div>
        </CardContent>
      </Card>

      <Card variant="elevated">
        <CardContent className="p-6">
          <div className="flex items-center justify-between">
            <div>
              <p className="text-sm text-text-secondary">检查项</p>
              <p className="text-2xl font-bold text-text-primary">
                {safetyChecks.length}
              </p>
            </div>
            <div className="p-3 rounded-full bg-accent/20">
              <FileText className="w-6 h-6 text-accent" />
            </div>
          </div>
        </CardContent>
      </Card>

      <Card variant="elevated">
        <CardContent className="p-6">
          <div className="flex items-center justify-between">
            <div>
              <p className="text-sm text-text-secondary">通过率</p>
              <p className="text-2xl font-bold text-text-primary">
                {passRate}%
              </p>
            </div>
            <div className="p-3 rounded-full bg-success/20">
              <CheckCircle className="w-6 h-6 text-success" />
            </div>
          </div>
        </CardContent>
      </Card>
    </div>
  );
}