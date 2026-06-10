import { AlertTriangle } from 'lucide-react';
import type { FolderPolicy } from '../../../types';

const POLICIES: Array<{ id: FolderPolicy; label: string; desc: string; highRisk: boolean }> = [
  { id: 'KeepOriginalStructure', label: '保持原结构', desc: '原来有文件夹保留，没有不建', highRisk: false },
  { id: 'OneMovieOneFolder', label: '一片一夹', desc: '没有文件夹的电影自动创建文件夹', highRisk: false },
  { id: 'NormalizeExistingFolders', label: '规范文件夹名', desc: '已有文件夹也按规则重命名', highRisk: false },
  { id: 'ChineseFolderPtFile', label: '中文文件夹+PT文件', desc: '文件夹中文，文件保留PT原名', highRisk: false },
  { id: 'Flatten', label: '去除文件夹', desc: '移出到上级目录（高风险）', highRisk: true },
  { id: 'TvShowStructure', label: '剧集目录结构', desc: 'show/season 层级', highRisk: false },
  { id: 'NoFolder', label: '不建文件夹', desc: '只改名，不创建/移动文件夹', highRisk: false },
];

export interface FolderPolicySelectorProps {
  selected: FolderPolicy;
  onSelect: (policy: FolderPolicy) => void;
}

export function FolderPolicySelector({ selected, onSelect }: FolderPolicySelectorProps) {
  return (
    <div className="flex flex-wrap gap-1.5">
      {POLICIES.map((p) => (
        <button
          key={p.id}
          className={`flex items-center gap-1.5 px-2.5 py-1.5 rounded-lg text-xs transition-colors ${selected === p.id ? 'bg-primary text-white' : 'bg-surface-hover text-text-secondary hover:text-text-primary border border-transparent hover:border-border'}`}
          onClick={() => onSelect(p.id)}
          title={p.desc}
        >
          {p.highRisk && <AlertTriangle className="h-3 w-3" />}
          {p.label}
        </button>
      ))}
    </div>
  );
}
