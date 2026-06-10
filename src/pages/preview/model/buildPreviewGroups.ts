import type { RenamePreviewItem, FolderGroup, PreviewFileItem, GroupMediaType, FileRole, GroupStatus, PreviewPlanTree } from '../../../types';

export function buildPreviewGroups(previews: RenamePreviewItem[]): PreviewPlanTree {
  const groupMap = new Map<string, RenamePreviewItem[]>();

  for (const item of previews) {
    const parentDir = getParentDir(item.source_path);
    const existing = groupMap.get(parentDir) || [];
    existing.push(item);
    groupMap.set(parentDir, existing);
  }

  const groups: FolderGroup[] = [];
  let groupId = 0;

  for (const [parentDir, items] of groupMap) {
    groupId++;

    const children = items.map((item) => toFileItem(item));
    const status = aggregateGroupStatus(children, false);
    const mediaType = inferGroupMediaType(items);
    const folderName = extractFolderName(parentDir);
    const targetFolder = inferTargetFolder(children);

    groups.push({
      id: `group_${String(groupId).padStart(3, '0')}`,
      media_type: mediaType,
      original_folder_name: folderName,
      target_folder_name: targetFolder,
      original_path: parentDir,
      target_path: parentDir.replace(folderName, targetFolder),
      file_count: children.length,
      tmdb_match_status: 'NotSearched',
      folder_policy: null,
      naming_preset: 'clean-library',
      status,
      should_skip: false,
      needs_manual_review: items.some((i) => i.needs_manual_review),
      children,
    });
  }

  return {
    groups,
    total_files: previews.length,
    total_groups: groups.length,
    skipped_groups: 0,
    blocked_groups: 0,
  };
}

function toFileItem(item: RenamePreviewItem): PreviewFileItem {
  return {
    id: item.id,
    file_role: inferFileRole(item.parsed_info.media_item.file_name, item.parsed_info.media_item.extension),
    original_name: item.parsed_info.media_item.file_name,
    target_name: item.proposed_name,
    extension: normalizeExt(item.parsed_info.media_item.extension),
    subtitle_language: null,
    original_path: item.source_path,
    target_path: item.target_path,
    should_skip: item.should_skip,
    needs_manual_review: item.needs_manual_review,
    metadata_source: item.metadata_source,
    safety_status: item.should_skip ? 'Skipped' : item.needs_manual_review ? 'NeedsReview' : item.conflicts.some((c) => c.blocking) ? 'Blocker' : item.conflicts.length > 0 ? 'Conflict' : 'Ready',
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
  const types = items.map((i) => i.media_type);
  const hasTv = types.some((t) => t === 'Series' || t === 'Anime');
  const hasMovie = types.some((t) => t === 'Movie');
  if (hasTv && hasMovie) return 'Mixed';
  if (hasTv) return 'Tv';
  if (hasMovie) return 'Movie';
  return 'Unknown';
}

function extractFolderName(path: string): string {
  return path.split('\\').pop() || path.split('/').pop() || path;
}

function inferTargetFolder(children: PreviewFileItem[]): string {
  const video = children.find((c) => c.file_role === 'MainVideo');
  const ref = video || children[0];
  if (!ref) return '';
  const name = ref.target_name;
  const dotIdx = name.lastIndexOf('.');
  return dotIdx > 0 ? name.substring(0, dotIdx) : name;
}

function inferFileRole(fileName: string, extension: string): FileRole {
  const lower = fileName.toLowerCase();
  const ext = extension.toLowerCase();
  if (['srt', 'ass', 'ssa', 'sub', 'idx', 'vtt'].includes(ext)) return 'Subtitle';
  if (ext === 'nfo' || ext === 'xml') return 'Nfo';
  if (['jpg', 'jpeg', 'png', 'gif', 'webp', 'bmp'].includes(ext)) {
    if (lower.includes('poster') || lower.includes('folder') || lower.includes('cover') || lower.includes('fanart') || lower.includes('backdrop')) return 'Image';
    return 'Extra';
  }
  if (['mkv', 'mp4', 'avi', 'm2ts', 'ts', 'mov', 'wmv', 'flv', 'webm', 'rmvb', 'iso', 'bdmv'].includes(ext)) return 'MainVideo';
  if (lower.includes('.zh.') || lower.includes('.chs.') || lower.includes('.cht.') || lower.includes('.en.')) return 'Subtitle';
  return 'Extra';
}

function normalizeExt(ext: string): string {
  const trimmed = ext.replace(/^\.+/, '');
  return trimmed ? `.${trimmed}` : '';
}

function getParentDir(filePath: string): string {
  const lastSep = Math.max(filePath.lastIndexOf('\\'), filePath.lastIndexOf('/'));
  return lastSep > 0 ? filePath.substring(0, lastSep) : filePath;
}

export { aggregateGroupStatus, inferFileRole };
