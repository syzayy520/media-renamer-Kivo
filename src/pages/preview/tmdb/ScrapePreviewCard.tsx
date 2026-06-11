import { Image } from 'lucide-react';
import { Badge, Card } from '../../../components/ui';
import { ScrapeWriteActions } from '../scrape/ScrapeWriteActions';
import type { ScrapePreviewCardProps } from './tmdbInspectorTypes';
import { TMDB_IMAGE_BASE } from './tmdbInspectorTypes';

export function ScrapePreviewCard({ selectedCandidate, selectedGroup }: ScrapePreviewCardProps) {
  const mainFile = selectedGroup?.children.find((child) => child.file_role === 'MainVideo') || selectedGroup?.children[0];

  return (
    <Card className="p-3">
      <div className="mb-2 flex items-center gap-1.5">
        <Image className="h-3.5 w-3.5 text-text-secondary" />
        <h3 className="text-xs font-semibold text-text-primary">刮削预览</h3>
      </div>
      <div className="space-y-2 text-xs text-text-secondary">
        <div><span className="text-text-tertiary">文件夹：</span>{selectedGroup?.target_folder_name}</div>
        <div><span className="text-text-tertiary">文件：</span>{mainFile?.target_name}</div>
        <div className="flex gap-2">
          <span className="text-text-tertiary">Poster：</span>
          {selectedCandidate.poster_path ? (
            <img src={`${TMDB_IMAGE_BASE}${selectedCandidate.poster_path}`} alt="Poster" className="h-24 w-16 rounded border border-border object-cover" />
          ) : (
            <span className="text-text-tertiary italic">无</span>
          )}
        </div>
        <div><span className="text-text-tertiary">标题：</span>{selectedCandidate.title} {selectedCandidate.year ? `(${selectedCandidate.year})` : ''}</div>
        <div><span className="text-text-tertiary">评分：</span>{selectedCandidate.vote_average != null ? `${selectedCandidate.vote_average.toFixed(1)} / 10` : '无'}</div>
        <div><span className="text-text-tertiary">简介：</span><span className="line-clamp-3">{selectedCandidate.overview || '无'}</span></div>
        <div><span className="text-text-tertiary">TMDb ID：</span>{selectedCandidate.tmdb_id}</div>
        <div><span className="text-text-tertiary">写入内容：</span><Badge variant="default" size="sm">NFO + 图片URL</Badge></div>
      </div>
      <ScrapeWriteActions candidate={selectedCandidate} group={selectedGroup} />
    </Card>
  );
}
