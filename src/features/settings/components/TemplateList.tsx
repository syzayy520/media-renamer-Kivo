// features/settings/components/TemplateList.tsx — 模板列表
// 职责：展示可编辑的模板列表

import type { RenameRule } from '../../../api/config/types';
import { TemplateEditor } from './TemplateEditor';

interface TemplateListProps {
  templates: RenameRule[];
  onSave: (mediaType: string, template: string) => Promise<void>;
}

export function TemplateList({ templates, onSave }: TemplateListProps) {
  if (templates.length === 0) {
    return (
      <div className="rounded-lg border border-white/5 bg-white/[0.02] p-6 text-center">
        <p className="text-sm text-white/30">暂无模板数据</p>
      </div>
    );
  }

  return (
    <div className="space-y-2">
      {templates.map((rule) => (
        <TemplateEditor
          key={rule.media_type + ':' + rule.template}
          rule={rule}
          onSave={onSave}
        />
      ))}
    </div>
  );
}
