import type { NamingRule } from '../../../types';
import { Badge } from '../../../components/ui';

export interface NamingTemplatePreviewProps {
  name: string;
  rule: NamingRule;
}

export function NamingTemplatePreview({ name, rule }: NamingTemplatePreviewProps) {
  return (
    <div className="rounded-lg border border-border bg-surface-subtle p-3">
      <div className="flex items-center gap-2 mb-2">
        <Badge variant="default" size="sm">{rule.name}</Badge>
        <span className="text-xs text-text-tertiary truncate">{rule.description}</span>
      </div>
      <div className="text-sm font-medium text-text-primary font-mono break-all">
        {name || <span className="text-text-tertiary italic">预览输出将显示在这里</span>}
      </div>
    </div>
  );
}
