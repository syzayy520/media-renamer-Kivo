import { Plus } from 'lucide-react';
import type { NamingToken } from '../../../types';
import { AVAILABLE_NAMING_TOKENS } from './namingPresetRules';

const TOKEN_LABELS: Record<NamingToken, string> = {
  ZhTitle: '中文标题', EnglishTitle: '英文标题', OriginalTitle: '原标题',
  OriginalNameWithoutExt: '原始文件名', OriginalReleaseName: 'PT发布名',
  Year: '年份', ReleaseDate: '发布日期', AirDate: '首播日期', SeasonYear: '季年份',
  Resolution: '分辨率', Source: '介质/来源', Edition: '剪辑/版本', Remux: 'REMUX',
  VideoCodec: '视频编码', VideoBitDepth: '位深', HdrFormat: 'HDR', DolbyVision: 'DV',
  AudioCodec: '音频编码', AudioChannels: '声道', AudioLanguage: '配音/语言', ReleaseGroup: '发布组',
  TmdbId: 'TMDb ID', ImdbId: 'IMDb ID', TvdbId: 'TVDB ID',
  ShowTitle: '剧名', Season: '季号', Episode: '集号', EpisodeTitle: '集标题',
  AbsoluteEpisode: '绝对集号', SeasonTitle: '季标题',
  Ext: '扩展名', SubtitleLanguage: '字幕语言', FileRole: '文件角色',
};

interface NamingTokenLibraryProps {
  onAdd: (token: NamingToken) => void;
}

export function NamingTokenLibrary({ onAdd }: NamingTokenLibraryProps) {
  return (
    <div className="grid grid-cols-2 gap-1">
      {AVAILABLE_NAMING_TOKENS.map((token) => (
        <button
          key={token}
          type="button"
          className="flex items-center justify-between rounded-lg bg-surface-hover px-2 py-1.5 text-left text-xs text-text-secondary hover:text-text-primary"
          onClick={() => onAdd(token)}
        >
          <span className="truncate">{TOKEN_LABELS[token]}</span>
          <Plus className="h-3 w-3 shrink-0" />
        </button>
      ))}
    </div>
  );
}
