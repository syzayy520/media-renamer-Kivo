import type {
  RenamePreviewItem,
  FolderGroup,
  PreviewFileItem,
  GroupMediaType,
  FileRole,
  GroupStatus,
  PreviewPlanTree,
} from '../../../types';

const VIDEO_EXTENSIONS = ['mkv', 'mp4', 'avi', 'm2ts', 'ts', 'mov', 'wmv', 'flv', 'webm', 'rmvb', 'iso', 'bdmv'];
const SUBTITLE_EXTENSIONS = ['srt', 'ass', 'ssa', 'sub', 'idx', 'vtt'];
const IMAGE_EXTENSIONS = ['jpg', 'jpeg', 'png', 'gif', 'webp', 'bmp'];

export function buildPreviewGroups(previews: RenamePreviewItem[], scanRoot?: string): PreviewPlanTree {
  const root = normalizeDir(scanRoot || detectScanRoot(previews));
  const parentGroups = new Map<string, RenamePreviewItem[]>();

  for (const item of previews) {
    const parentDir = getParentDir(item.source_path);
    const existing = parentGroups.get(parentDir) || [];
    existing.push(item);
    parentGroups.set(parentDir, existing);
  }

  let groupId = 0;
  const groups: FolderGroup[] = [];

  for (const [parentDir, items] of parentGroups) {
    const slices = splitParentDirectoryIntoMediaGroups(items, root);

    for (const slice of slices) {
      groupId += 1;
      groups.push(createFolderGroup(groupId, parentDir, slice.items, slice.originalName));
    }
  }

  const sortedGroups = groups.sort((a, b) => a.target_folder_name.localeCompare(b.target_folder_name, 'zh-Hans-CN'));

  return {
    groups: sortedGroups,
    total_files: previews.length,
    total_groups: sortedGroups.length,
    skipped_groups: sortedGroups.filter((g) => g.status === 'Skipped').length,
    blocked_groups: sortedGroups.filter((g) => g.status === 'Blocker').length,
  };
}

interface MediaSlice {
  originalName: string;
  items: RenamePreviewItem[];
}

function splitParentDirectoryIntoMediaGroups(items: RenamePreviewItem[], scanRoot: string): MediaSlice[] {
  if (items.length <= 1) {
    return [{ originalName: inferOriginalGroupName(items), items }];
  }

  if (isStrongTvGroup(items)) {
    return [{ originalName: inferOriginalGroupName(items), items }];
  }

  const slices = new Map<string, MediaSlice>();
  const companions: RenamePreviewItem[] = [];

  for (const item of items) {
    if (isVideoItem(item)) {
      const key = mediaIdentityKey(item);
      const originalName = inferOriginalGroupName([item]);
      const existing = slices.get(key) || { originalName, items: [] };
      existing.items.push(item);
      slices.set(key, existing);
    } else {
      companions.push(item);
    }
  }

  for (const companion of companions) {
    const companionKey = mediaIdentityKey(companion);
    const matchedKey = findBestCompanionMatch(companionKey, slices);

    if (matchedKey) {
      slices.get(matchedKey)!.items.push(companion);
      continue;
    }

    const originalName = inferOriginalGroupName([companion]);
    const existing = slices.get(companionKey) || { originalName, items: [] };
    existing.items.push(companion);
    slices.set(companionKey, existing);
  }

  if (slices.size === 0) {
    return [{ originalName: inferOriginalGroupName(items), items }];
  }

  const parentDir = getParentDir(items[0].source_path);
  const isWorkspaceRoot = normalizeDir(parentDir) === scanRoot;

  if (slices.size === 1 && !isWorkspaceRoot) {
    const only = Array.from(slices.values())[0];
    return [{ originalName: only.originalName, items: only.items }];
  }

  return Array.from(slices.values());
}

function createFolderGroup(groupId: number, parentDir: string, items: RenamePreviewItem[], originalName: string): FolderGroup {
  const children = items.map((item) => toFileItem(item));
  const status = aggregateGroupStatus(children, false);
  const targetFolder = inferTargetFolder(children, originalName);
  const mediaType = inferGroupMediaType(items);

  return {
    id: `group_${String(groupId).padStart(3, '0')}`,
    media_type: mediaType,
    original_folder_name: originalName,
    target_folder_name: targetFolder,
    original_path: parentDir,
    target_path: joinPath(parentDir, targetFolder),
    file_count: children.length,
    tmdb_match_status: 'NotSearched',
    folder_policy: null,
    naming_preset: 'clean-library',
    status,
    should_skip: false,
    needs_manual_review: items.some((i) => i.needs_manual_review),
    children,
  };
}

function findBestCompanionMatch(companionKey: string, slices: Map<string, MediaSlice>): string | null {
  for (const key of slices.keys()) {
    if (companionKey === key || companionKey.startsWith(key) || key.startsWith(companionKey)) {
      return key;
    }
  }
  return null;
}

function isStrongTvGroup(items: RenamePreviewItem[]): boolean {
  const episodeLike = items.filter((item) => hasEpisodeSignal(item));
  if (episodeLike.length < 2) {
    return false;
  }

  const showKeys = new Set(episodeLike.map((item) => normalizeKey(preferredParsedTitle(item) || inferTitleFromName(item))));
  return showKeys.size <= 2;
}

function hasEpisodeSignal(item: RenamePreviewItem): boolean {
  if (item.parsed_info.episode != null || item.parsed_info.season != null) {
    return true;
  }

  const name = `${item.original_name} ${item.proposed_name} ${item.parsed_info.media_item.file_name}`;
  return /s\d{1,2}e\d{1,3}/i.test(name)
    || /\b\d{1,2}x\d{1,3}\b/i.test(name)
    || /\bep?\d{1,3}\b/i.test(name)
    || /第\s*\d{1,3}\s*[集话話]/.test(name);
}

function mediaIdentityKey(item: RenamePreviewItem): string {
  const title = inferTitleFromName(item) || preferredParsedTitle(item) || getBasename(item.parsed_info.media_item.file_name);
  const year = item.parsed_info.year || inferYearFromName(item.original_name) || inferYearFromName(item.proposed_name) || '';
  return normalizeKey(`${title}-${year}`);
}

function preferredParsedTitle(item: RenamePreviewItem): string {
  const title = item.parsed_info.title || '';
  return cleanTitleText(title);
}

function inferTitleFromName(item: RenamePreviewItem): string {
  const candidate = item.proposed_name || item.original_name || item.parsed_info.media_item.file_name;
  return cleanTitleText(stripExtension(candidate));
}

function cleanTitleText(value: string): string {
  const normalized = value
    .replace(/^\.+/, '')
    .replace(/[._]+/g, ' ')
    .replace(/\[[^\]]*\]/g, ' ')
    .replace(/\s+/g, ' ')
    .trim();

  const yearMatch = normalized.match(/^(.*?)(?:\s*\(?((?:19|20)\d{2})\)?)(?:\s|$)/);
  const beforeYear = yearMatch?.[1]?.trim();
  if (beforeYear) {
    return beforeYear.replace(/\s+/g, ' ').trim();
  }

  return normalized
    .replace(/\b(720p|1080p|2160p|4k|bluray|blu ray|blu-ray|web dl|web-dl|webrip|hdtv|remux|unrated|proper|repack|extended|gb|gbr|usa|x264|x265|hevc|h264|h265|vc1|vc|10bit|8bit|hdr|dv)\b/gi, ' ')
    .replace(/\s+/g, ' ')
    .trim();
}

function inferOriginalGroupName(items: RenamePreviewItem[]): string {
  const ref = items.find((item) => isVideoItem(item)) || items[0];
  if (!ref) return '';
  const title = inferTitleFromName(ref) || preferredParsedTitle(ref);
  const year = ref.parsed_info.year || inferYearFromName(ref.original_name) || inferYearFromName(ref.proposed_name);
  if (title && year) return `${title} (${year})`;
  if (title) return title;
  return stripExtension(ref.original_name || ref.parsed_info.media_item.file_name).replace(/^\.+/, '');
}

function inferYearFromName(name: string): number | null {
  const match = name.match(/(?:^|[^0-9])((?:19|20)\d{2})(?:[^0-9]|$)/);
  return match ? Number(match[1]) : null;
}

function toFileItem(item: RenamePreviewItem): PreviewFileItem {
  return {
    id: item.id,
    file_role: inferFileRole(item.parsed_info.media_item.file_name, item.parsed_info.media_item.extension),
    original_name: item.parsed_info.media_item.file_name,
    target_name: item.proposed_name,
    extension: normalizeExt(item.parsed_info.media_item.extension),
    subtitle_language: inferSubtitleLanguage(item.parsed_info.media_item.file_name),
    original_path: item.source_path,
    target_path: item.target_path,
    should_skip: item.should_skip,
    needs_manual_review: item.needs_manual_review,
    metadata_source: item.metadata_source,
    safety_status: item.should_skip
      ? 'Skipped'
      : item.needs_manual_review
        ? 'NeedsReview'
        : item.conflicts.some((c) => c.blocking)
          ? 'Blocker'
          : item.conflicts.length > 0
            ? 'Conflict'
            : 'Ready',
    confidence: item.confidence,
  };
}

function aggregateGroupStatus(children: PreviewFileItem[], groupSkipped: boolean): GroupStatus {
  if (groupSkipped) return 'Skipped';
  if (children.length === 0) return 'Ready';

  const statuses = children.map((c) => c.safety_status);
  if (statuses.some((s) => s === 'Blocker')) return 'Blocker';
  if (statuses.every((s) => s === 'Skipped')) return 'Skipped';
  if (statuses.some((s) => s === 'Skipped')) return 'PartialSkipped';
  if (statuses.some((s) => s === 'Conflict')) return 'Conflict';
  if (statuses.some((s) => s === 'NeedsReview')) return 'NeedsReview';
  return 'Ready';
}

function inferGroupMediaType(items: RenamePreviewItem[]): GroupMediaType {
  if (isStrongTvGroup(items)) {
    return items.some((item) => item.media_type === 'Anime') ? 'Anime' : 'Tv';
  }

  const types = items.map((i) => i.media_type);
  const hasAnime = types.some((t) => t === 'Anime');
  const hasMovie = types.some((t) => t === 'Movie');
  if (hasAnime && !hasMovie) return 'Anime';
  if (hasMovie || items.some((item) => isVideoItem(item))) return 'Movie';
  return 'Unknown';
}

function inferTargetFolder(children: PreviewFileItem[], fallbackName: string): string {
  const video = children.find((c) => c.file_role === 'MainVideo');
  const ref = video || children[0];
  if (!ref) return fallbackName;

  const fromTarget = stripExtension(ref.target_name).replace(/^\.+/, '').replace(/\.+$/, '').trim();
  if (!fromTarget || looksLikeRawReleaseName(fromTarget)) {
    return fallbackName;
  }

  return fromTarget;
}

function looksLikeRawReleaseName(name: string): boolean {
  const lowered = name.toLowerCase();
  const technicalTokens = ['1080p', '2160p', '720p', 'bluray', 'remux', 'web-dl', 'webrip', 'hdtv', 'x264', 'x265', 'hevc', 'truehd', 'dts'];
  return name.startsWith('.') || technicalTokens.filter((token) => lowered.includes(token)).length >= 2;
}

function inferFileRole(fileName: string, extension: string): FileRole {
  const lower = fileName.toLowerCase();
  const ext = extension.toLowerCase().replace(/^\.+/, '');
  if (SUBTITLE_EXTENSIONS.includes(ext)) return 'Subtitle';
  if (ext === 'nfo' || ext === 'xml') return 'Nfo';
  if (IMAGE_EXTENSIONS.includes(ext)) {
    if (lower.includes('poster') || lower.includes('folder') || lower.includes('cover') || lower.includes('fanart') || lower.includes('backdrop')) return 'Image';
    return 'Extra';
  }
  if (VIDEO_EXTENSIONS.includes(ext)) return 'MainVideo';
  if (lower.includes('.zh.') || lower.includes('.chs.') || lower.includes('.cht.') || lower.includes('.en.')) return 'Subtitle';
  return 'Extra';
}

function inferSubtitleLanguage(fileName: string): string | null {
  const lower = fileName.toLowerCase();
  if (lower.includes('.zh.') || lower.includes('.chs.')) return 'zh';
  if (lower.includes('.cht.')) return 'zh-Hant';
  if (lower.includes('.en.')) return 'en';
  return null;
}

function isVideoItem(item: RenamePreviewItem): boolean {
  const ext = item.parsed_info.media_item.extension.toLowerCase().replace(/^\.+/, '');
  return item.parsed_info.media_item.is_video || VIDEO_EXTENSIONS.includes(ext);
}

function stripExtension(name: string): string {
  const clean = name.trim();
  const lastSlash = Math.max(clean.lastIndexOf('\\'), clean.lastIndexOf('/'));
  const lastDot = clean.lastIndexOf('.');
  if (lastDot > Math.max(lastSlash, 0)) {
    return clean.substring(0, lastDot);
  }
  return clean;
}

function normalizeExt(ext: string): string {
  const trimmed = ext.replace(/^\.+/, '');
  return trimmed ? `.${trimmed}` : '';
}

function getParentDir(filePath: string): string {
  const lastSep = Math.max(filePath.lastIndexOf('\\'), filePath.lastIndexOf('/'));
  return lastSep > 0 ? filePath.substring(0, lastSep) : filePath;
}

function getBasename(fileName: string): string {
  const lastDot = fileName.lastIndexOf('.');
  return lastDot > 0 ? fileName.substring(0, lastDot) : fileName;
}

function joinPath(parentDir: string, childName: string): string {
  if (!parentDir) return childName;
  const separator = parentDir.includes('\\') ? '\\' : '/';
  return `${parentDir.replace(/[\\/]+$/, '')}${separator}${childName}`;
}

function normalizeDir(path: string): string {
  return path.replace(/\//g, '\\').replace(/\\+$/, '');
}

function normalizeKey(value: string): string {
  return value.toLowerCase().replace(/[^\p{L}\p{N}]+/gu, '').trim();
}

function detectScanRoot(previews: RenamePreviewItem[]): string {
  if (previews.length === 0) return '';

  const dirs = previews.map((item) => normalizeDir(getParentDir(item.source_path)));
  let root = dirs[0] || '';

  for (const dir of dirs.slice(1)) {
    root = commonDirectoryPrefix(root, dir);
  }

  return root;
}

function commonDirectoryPrefix(left: string, right: string): string {
  const leftParts = normalizeDir(left).split('\\');
  const rightParts = normalizeDir(right).split('\\');
  const parts: string[] = [];

  for (let index = 0; index < Math.min(leftParts.length, rightParts.length); index += 1) {
    if (leftParts[index].toLowerCase() !== rightParts[index].toLowerCase()) {
      break;
    }
    parts.push(leftParts[index]);
  }

  return parts.join('\\');
}

export { aggregateGroupStatus, inferFileRole };
