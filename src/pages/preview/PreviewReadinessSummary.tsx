// pages/preview/PreviewReadinessSummary 组件
// 职责：展示预览列表的整体准备状态统计

import { useMemo } from 'react';
import { FileCheck, AlertTriangle, ShieldAlert, SkipForward, Search, CheckCircle2, Info } from 'lucide-react';
import { Badge } from '../../components/ui';
import type { RenamePreviewItem, SafetyReport } from '../../types';
import type { ItemSearchState } from '../../state/tmdbSearchStore';

interface PreviewReadinessSummaryProps {
  previews: RenamePreviewItem[];
  itemStates: Record<string, ItemSearchState>;
  safety: SafetyReport | null;
  tmdbEnabled: boolean;
}

interface ReadinessStats {
  total: number;
  ready: number;
  hasCandidates: number;
  applied: number;
  needsReview: number;
  hasConflicts: number;
  skipped: number;
  blockers: number;
  warnings: number;
}

function computeStats(
  previews: RenamePreviewItem[],
  itemStates: Record<string, ItemSearchState>,
  safety: SafetyReport | null,
): ReadinessStats {
  let ready = 0;
  let hasCandidates = 0;
  let applied = 0;
  let needsReview = 0;
  let hasConflicts = 0;
  let skipped = 0;

  for (const item of previews) {
    if (item.should_skip) {
      skipped++;
      continue;
    }
    if (item.needs_manual_review) {
      needsReview++;
    }
    if (item.conflicts.length > 0) {
      hasConflicts++;
    }
    if (item.metadata_source === 'Tmdb') {
      applied++;
    }
    const state = itemStates[item.id];
    if (state && state.results.length > 0) {
      hasCandidates++;
    }
    if (!item.should_skip && !item.needs_manual_review && item.conflicts.length === 0) {
      ready++;
    }
  }

  const blockers = safety?.blocking_reasons.length ?? 0;
  const warnings = safety?.checks.filter((c) => !c.passed).length ?? 0;

  return {
    total: previews.length,
    ready,
    hasCandidates,
    applied,
    needsReview,
    hasConflicts,
    skipped,
    blockers,
    warnings,
  };
}

export function PreviewReadinessSummary({
  previews,
  itemStates,
  safety,
  tmdbEnabled,
}: PreviewReadinessSummaryProps) {
  const stats = useMemo(
    () => computeStats(previews, itemStates, safety),
    [previews, itemStates, safety],
  );

  if (stats.total === 0) return null;

  const canExecute = safety?.can_execute !== false;

  return (
    <div className="mb-6 p-4 bg-bg-card rounded-lg border border-border">
      <div className="flex items-center justify-between mb-3">
        <h3 className="text-sm font-medium text-text-primary flex items-center gap-2">
          <Info className="w-4 h-4 text-accent" />
          准备状态概览
        </h3>
        <Badge variant={canExecute ? 'success' : 'danger'} size="sm">
          {canExecute ? '可执行' : '有阻断'}
        </Badge>
      </div>

      <div className="grid grid-cols-2 sm:grid-cols-4 lg:grid-cols-8 gap-3">
        <StatCard
          icon={<FileCheck className="w-4 h-4" />}
          label="总计"
          value={stats.total}
          variant="default"
        />
        <StatCard
          icon={<CheckCircle2 className="w-4 h-4" />}
          label="就绪"
          value={stats.ready}
          variant="success"
        />
        {tmdbEnabled && (
          <>
            <StatCard
              icon={<Search className="w-4 h-4" />}
              label="有候选"
              value={stats.hasCandidates}
              variant="info"
            />
            <StatCard
              icon={<CheckCircle2 className="w-4 h-4" />}
              label="已应用"
              value={stats.applied}
              variant="accent"
            />
          </>
        )}
        <StatCard
          icon={<AlertTriangle className="w-4 h-4" />}
          label="需审核"
          value={stats.needsReview}
          variant="warning"
        />
        <StatCard
          icon={<ShieldAlert className="w-4 h-4" />}
          label="有冲突"
          value={stats.hasConflicts}
          variant="danger"
        />
        <StatCard
          icon={<SkipForward className="w-4 h-4" />}
          label="跳过"
          value={stats.skipped}
          variant="default"
        />
        {stats.blockers > 0 && (
          <StatCard
            icon={<ShieldAlert className="w-4 h-4" />}
            label="阻断"
            value={stats.blockers}
            variant="danger"
          />
        )}
      </div>
    </div>
  );
}

interface StatCardProps {
  icon: React.ReactNode;
  label: string;
  value: number;
  variant: 'default' | 'success' | 'info' | 'warning' | 'danger' | 'accent';
}

function StatCard({ icon, label, value, variant }: StatCardProps) {
  const colorMap = {
    default: 'text-text-secondary bg-bg-secondary',
    success: 'text-success bg-success/10',
    info: 'text-info bg-info/10',
    warning: 'text-warning bg-warning/10',
    danger: 'text-error bg-error/10',
    accent: 'text-accent bg-accent/10',
  };

  return (
    <div className={`flex items-center gap-2 p-2 rounded-md ${colorMap[variant]}`}>
      {icon}
      <div className="min-w-0">
        <p className="text-lg font-bold leading-none">{value}</p>
        <p className="text-xs opacity-80 mt-0.5">{label}</p>
      </div>
    </div>
  );
}
