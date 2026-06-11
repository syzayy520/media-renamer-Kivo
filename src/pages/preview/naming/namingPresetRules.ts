import type { NamingRule, NamingToken, Separator, TokenConfig, TitleStrategy } from '../../../types';

export interface NamingPresetDefinition {
  id: string;
  label: string;
  desc: string;
  titleStrategy: TitleStrategy;
  rule: NamingRule;
}

export const AVAILABLE_NAMING_TOKENS: NamingToken[] = [
  'ZhTitle', 'EnglishTitle', 'OriginalTitle', 'OriginalReleaseName',
  'Year', 'Season', 'Episode', 'Resolution', 'Source', 'Edition', 'Remux',
  'HdrFormat', 'DolbyVision', 'VideoCodec', 'VideoBitDepth',
  'AudioLanguage', 'AudioCodec', 'AudioChannels', 'ReleaseGroup', 'Ext',
];

export const NAMING_PRESETS: NamingPresetDefinition[] = [
  {
    id: 'clean-library',
    label: '清爽媒体库',
    desc: '中文标题 (年份).mkv',
    titleStrategy: 'ChineseOnly',
    rule: rule('clean-library', '清爽媒体库', [
      token('ZhTitle'),
      token('Year', { separator: 'Space', wrapper: 'Parentheses' }),
      token('Ext', { separator: 'None' }),
    ]),
  },
  {
    id: 'pt-0day-movie',
    label: 'PT 0day 电影',
    desc: 'First.Blood.1982.1080p.BluRay.x265.DTS.5.1-PTer.mkv',
    titleStrategy: 'EnglishOnly',
    rule: rule('pt-0day-movie', 'PT 0day 电影：英文名.年份.分辨率.介质.视频.音频.声道-组', [
      token('EnglishTitle', { case_strategy: 'PtDotStyle' }),
      token('Year', { separator: 'Dot', empty_policy: 'NeedsReview' }),
      token('Resolution', { separator: 'Dot', empty_policy: 'NeedsReview' }),
      token('Source', { separator: 'Dot', empty_policy: 'NeedsReview' }),
      token('VideoCodec', { separator: 'Dot', empty_policy: 'NeedsReview' }),
      token('AudioCodec', { separator: 'Dot' }),
      token('AudioChannels', { separator: 'Dot' }),
      token('ReleaseGroup', { separator: 'None', prefix: '-' }),
      token('Ext', { separator: 'None' }),
    ]),
  },
  {
    id: 'pt-0day-video-postfix',
    label: 'PT 外站视频后置',
    desc: 'First.Blood.1982.1080p.BluRay.DTS.5.1.x265-PTer.mkv',
    titleStrategy: 'EnglishOnly',
    rule: rule('pt-0day-video-postfix', 'Encode/WEB 常见：音频在前，视频编码后置', [
      token('EnglishTitle', { case_strategy: 'PtDotStyle' }),
      token('Year', { separator: 'Dot', empty_policy: 'NeedsReview' }),
      token('Resolution', { separator: 'Dot', empty_policy: 'NeedsReview' }),
      token('Source', { separator: 'Dot', empty_policy: 'NeedsReview' }),
      token('AudioCodec', { separator: 'Dot' }),
      token('AudioChannels', { separator: 'Dot' }),
      token('VideoCodec', { separator: 'Dot', empty_policy: 'NeedsReview' }),
      token('ReleaseGroup', { separator: 'None', prefix: '-' }),
      token('Ext', { separator: 'None' }),
    ]),
  },
  {
    id: 'pt-original-release',
    label: 'PT 原始发布名',
    desc: '完整保留原始 PT/BT 发布名.mkv',
    titleStrategy: 'ChinesePrefixPt',
    rule: rule('pt-original-release', '完整保留原始发布名，只补扩展名', [
      token('OriginalReleaseName'),
      token('Ext', { separator: 'None' }),
    ]),
  },
  {
    id: 'chinese-prefix-pt',
    label: '中文前缀 + PT',
    desc: '第一滴血.First.Blood.1982.1080p.BluRay.x265-PTer.mkv',
    titleStrategy: 'ChinesePrefixPt',
    rule: rule('chinese-prefix-pt', '中文名放最前，后面接原始 PT/BT 发布名', [
      token('ZhTitle'),
      token('OriginalReleaseName', { separator: 'Dot' }),
      token('Ext', { separator: 'None' }),
    ]),
  },
  {
    id: 'bilingual-library',
    label: '中英双语',
    desc: '中文 - English (年份).mkv',
    titleStrategy: 'Bilingual',
    rule: rule('bilingual-library', '中英双语媒体库', [
      token('ZhTitle'),
      token('EnglishTitle', { separator: 'Space', prefix: '- ' }),
      token('Year', { separator: 'Space', wrapper: 'Parentheses' }),
      token('Ext', { separator: 'None' }),
    ]),
  },
  {
    id: 'jellyfin-emby',
    label: 'Jellyfin/Emby',
    desc: '中文标题 (年份).mkv',
    titleStrategy: 'ChineseOnly',
    rule: rule('jellyfin-emby', '媒体库兼容命名', [
      token('ZhTitle'),
      token('Year', { separator: 'Space', wrapper: 'Parentheses' }),
      token('Ext', { separator: 'None' }),
    ]),
  },
];

export function buildNamingPresetRule(id: string): NamingRule {
  return cloneRule(NAMING_PRESETS.find((preset) => preset.id === id)?.rule ?? NAMING_PRESETS[0].rule);
}

export function getNamingPresetStrategy(id: string): TitleStrategy {
  return NAMING_PRESETS.find((preset) => preset.id === id)?.titleStrategy ?? 'ChineseOnly';
}

export function appendToken(rule: NamingRule, tokenName: NamingToken): NamingRule {
  const separator: Separator = rule.tokens.length === 0 ? 'None' : inferDefaultSeparator(rule);
  return {
    ...rule,
    tokens: [...rule.tokens, token(tokenName, { separator })],
  };
}

function inferDefaultSeparator(rule: NamingRule): Separator {
  if (rule.name.startsWith('pt-') || rule.name.includes('pt')) return 'Dot';
  return 'Space';
}

function rule(name: string, description: string, tokens: TokenConfig[]): NamingRule {
  return { name, description, tokens };
}

function token(tokenName: NamingToken, overrides: Partial<TokenConfig> = {}): TokenConfig {
  return {
    token: tokenName,
    prefix: '',
    suffix: '',
    separator: 'Space',
    empty_policy: 'Hide',
    case_strategy: 'AsIs',
    wrapper: 'None',
    enabled: true,
    ...overrides,
  };
}

function cloneRule(source: NamingRule): NamingRule {
  return {
    ...source,
    tokens: source.tokens.map((item) => ({ ...item })),
  };
}
