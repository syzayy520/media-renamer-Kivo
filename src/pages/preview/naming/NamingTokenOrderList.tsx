import { ChevronDown, ChevronUp, X } from 'lucide-react';
import type { NamingToken, TokenConfig } from '../../../types';
import { NamingTokenLibrary } from './NamingTokenLibrary';

const TOKEN_LABELS: Record<NamingToken, string> = {
  ZhTitle: '中文标题', EnglishTitle: '英文标题', OriginalTitle: '原标题',
  OriginalNameWithoutExt: '原始文件名', OriginalReleaseName: 'PT发布名',
  Year: '年份', ReleaseDate: '发布日期', AirDate: '首播日期', SeasonYear: '季年份',
  Resolution: '分辨率', Source: '来源', Edition: '版本', Remux: 'Remux',
  VideoCodec: '视频编码', VideoBitDepth: '位深', HdrFormat: 'HDR', DolbyVision: 'DV',
  AudioCodec: '音频编码', AudioChannels: '声道', AudioLanguage: '音频语言', ReleaseGroup: '发布组',
  TmdbId: 'TMDb ID', ImdbId: 'IMDb ID', TvdbId: 'TVDB ID',
  ShowTitle: '剧名', Season: '季号', Episode: '集号', EpisodeTitle: '集标题',
  AbsoluteEpisode: '绝对集号', SeasonTitle: '季标题',
  Ext: '扩展名', SubtitleLanguage: '字幕语言', FileRole: '文件角色',
};

export interface NamingTokenOrderListProps {
  tokens: TokenConfig[];
  onReorder: (tokens: TokenConfig[]) => void;
  onRemove: (index: number) => void;
}

export function NamingTokenOrderList({ tokens, onReorder, onRemove }: NamingTokenOrderListProps) {
  const moveUp = (idx: number) => {
    if (idx <= 0) return;
    const newTokens = [...tokens];
    [newTokens[idx - 1], newTokens[idx]] = [newTokens[idx], newTokens[idx - 1]];
    onReorder(newTokens);
  };

  const moveDown = (idx: number) => {
    if (idx >= tokens.length - 1) return;
    const newTokens = [...tokens];
    [newTokens[idx], newTokens[idx + 1]] = [newTokens[idx + 1], newTokens[idx]];
    onReorder(newTokens);
  };

  const addToken = (token: NamingToken) => {
    onReorder([...tokens, buildTokenConfig(token, defaultSeparator(tokens))]);
  };

  return (
    <div className="space-y-2">
      <div className="rounded-lg border border-border/70 p-2">
        <div className="mb-1.5 text-xs font-medium text-text-secondary">添加 Token</div>
        <NamingTokenLibrary onAdd={addToken} />
      </div>

      {tokens.length === 0 ? (
        <div className="py-4 text-center text-xs text-text-tertiary">未添加 Token</div>
      ) : (
        <div className="flex max-h-48 flex-col gap-0.5 overflow-y-auto">
          {tokens.map((token, idx) => (
            <div key={`${token.token}-${idx}`} className="flex items-center gap-1 rounded px-2 py-1 transition-colors hover:bg-surface-hover">
              <span className="flex-1 truncate text-xs text-text-primary">
                {TOKEN_LABELS[token.token] || token.token}
              </span>
              <div className="flex shrink-0 items-center gap-0.5">
                <button className="p-0.5 text-text-tertiary hover:text-text-primary" onClick={() => moveUp(idx)} disabled={idx === 0}>
                  <ChevronUp className="h-3 w-3" />
                </button>
                <button className="p-0.5 text-text-tertiary hover:text-text-primary" onClick={() => moveDown(idx)} disabled={idx === tokens.length - 1}>
                  <ChevronDown className="h-3 w-3" />
                </button>
                <button className="p-0.5 text-text-tertiary hover:text-error" onClick={() => onRemove(idx)}>
                  <X className="h-3 w-3" />
                </button>
              </div>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}

function defaultSeparator(tokens: TokenConfig[]): TokenConfig['separator'] {
  return tokens.length === 0 ? 'None' : 'Dot';
}

function buildTokenConfig(token: NamingToken, separator: TokenConfig['separator']): TokenConfig {
  return {
    token,
    prefix: '',
    suffix: '',
    separator,
    empty_policy: 'Hide',
    case_strategy: token === 'EnglishTitle' ? 'PtDotStyle' : 'AsIs',
    wrapper: 'None',
    enabled: true,
  };
}
