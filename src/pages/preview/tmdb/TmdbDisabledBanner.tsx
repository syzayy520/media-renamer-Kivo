import { Globe, Settings } from 'lucide-react';
import { Button } from '../../../components/ui';

interface TmdbDisabledBannerProps {
  reason: string | null;
  onSettings: () => void;
}

export function TmdbDisabledBanner({ reason, onSettings }: TmdbDisabledBannerProps) {
  return (
    <div className="flex shrink-0 items-center gap-3 rounded-lg border border-warning/30 bg-warning/5 px-3 py-2">
      <Globe className="h-4 w-4 shrink-0 text-warning" />
      <span className="flex-1 text-xs text-text-secondary">TMDb 搜索暂未启用</span>
      {reason && <span className="max-w-xs truncate text-xs text-text-tertiary">{reason}</span>}
      <Button variant="secondary" size="sm" icon={<Settings className="h-3 w-3" />} onClick={onSettings}>去设置</Button>
    </div>
  );
}
