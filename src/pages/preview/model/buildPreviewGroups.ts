import type { RenamePreviewItem, FolderGroup, PreviewFileItem, GroupMediaType, FileRole, GroupStatus, PreviewPlanTree } from '../../../types';

/**
 * 从扫描结果构建真正的 Media Groups（而非将根目录文件捆绑为一个假组）。
 *
 * 规则：
 * 1. 子目录 → 每个子目录 = 一个 Media Group（已有行为保留）
 * 2. 根目录散装文件 → 按 basename 分组，每个独立视频 = 自己的 Media Group
 * 3. 剧集检测 → 多 SxxExx 同目录 = TV Group（不拆）
 * 4. Companion 文件 → 按同 basename + 同目录匹配到对应 video group
 *
 * @param previews 扫描后生成的预览项列表
 * @param scanRoot 扫描根目录路径
 */
export function buildPreviewGroups(previews: RenamePreviewItem[], scanRoot?: string): PreviewPlanTree {
  const root = normalizeDir(scanRoot || detectScanRoot(previews));

  // Step 1: 分离子目录文件和根目录文件
  const subdirFiles = new Map<string, RenamePreviewItem[]>();
  const rootFiles: RenamePreviewItem[] = [];

  for (const item of previews) {
    const parentDir = getParentDir(item.source_path);
    if (normalizeDir(parentDir) === root) {
      rootFiles.push(item);
    } else {
      const existing = subdirFiles.get(parentDir) || [];
      existing.push(item);
      subdirFiles.set(parentDir, existing);
    }
  }

  // Step 2: 构建 Media Groups
  const groups: FolderGroup[] = [];
  let groupId = 0;

  // 2a: 子目录组
  for (const [parentDir, items] of subdirFiles) {
    groupId++;
    const folderName = extractFolderName(parentDir);
    const isTv = detectTvGroup(items);
    const mediaType = isTv ? 'Tv' : inferGroupMediaType(items);

    const children = items.map((item) => toFileItem(item));
    const status = aggregateGroupStatus(children, false);
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

  // 2b: 根目录散装文件 → 按 basename 分组
  const rootGroups = groupRootFilesByBasename(rootFiles);
  for (const [basename, items] of rootGroups) {
    groupId++;
    const children = items.map((item) => toFileItem(item));
    const status = aggregateGroupStatus(children, false);
    const mediaType = inferGroupMediaType(items);
    // 对于散装文件，文件夹名就是 basename
    const targetFolder = inferTargetFolder(children);
    // 还原显示：用第一个文件的父目录名作为显示名
    const displayFolder = basename || extractFolderName(getParentDir(items[0].source_path));

    groups.push({
      id: `group_${String(groupId).padStart(3, '0')}`,
      media_type: mediaType,
      original_folder_name: displayFolder,
      target_folder_name: targetFolder,
      original_path: getParentDir(items[0].source_path),
      target_path: getParentDir(items[0].source_path).replace(displayFolder, targetFolder),
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

  // 如果没有子目录组也没有根文件组（极端情况），回退到原行为
  if (groups.length === 0 && subdirFiles.size === 0 && rootFiles.length > 0) {
    // 不应该出现但兜底
    const originalMap = new Map<string, RenamePreviewItem[]>();
    for (const item of previews) {
      const d = getParentDir(item.source_path);
      const e = originalMap.get(d) || [];
      e.push(item);
      originalMap.set(d, e);
    }
    for (const [dir, items] of originalMap) {
      groupId++;
      const children = items.map((item) => toFileItem(item));
      groups.push({
        id: `group_${String(groupId).padStart(3, '0')}`,
        media_type: inferGroupMediaType(items),
        original_folder_name: extractFolderName(dir),
        target_folder_name: inferTargetFolder(children),
        original_path: dir,
        target_path: dir,
        file_count: children.length,
        tmdb_match_status: 'NotSearched',
        folder_policy: null,
        naming_preset: 'clean-library',
        status: aggregateGroupStatus(children, false),
        should_skip: false,
        needs_manual_review: items.some((i) => i.needs_manual_review),
        children,
      });
    }
  }

  return {
    groups,
    total_files: previews.length,
    total_groups: groups.length,
    skipped_groups: groups.filter((g) => g.status === 'Skipped').length,
    blocked_groups: groups.filter((g) => g.status === 'Blocker').length,
  };
}

/**
 * 按 basename 将根目录散装文件分组
 * 主视频文件决定组名，companion 文件按同 basename 匹配
 */
function groupRootFilesByBasename(items: RenamePreviewItem[]): Map<string, RenamePreviewItem[]> {
  const groups = new Map<string, RenamePreviewItem[]>();

  // 先分离主视频和 companion 文件
  const videos: RenamePreviewItem[] = [];
  const companions: RenamePreviewItem[] = [];
  for (const item of items) {
    const ext = item.parsed_info.media_item.extension.toLowerCase();
    if (['mkv', 'mp4', 'avi', 'm2ts', 'ts', 'mov', 'wmv', 'flv', 'webm', 'rmvb', 'iso', 'bdmv'].includes(ext)) {
      videos.push(item);
    } else {
      companions.push(item);
    }
  }

  // 为每个视频创建组
  for (const video of videos) {
    const basename = getBasename(video.parsed_info.media_item.file_name);
    const existing = groups.get(basename) || [];
    existing.push(video);
    groups.set(basename, existing);
  }

  // 将 companion 文件匹配到对应 video 组
  for (const comp of companions) {
    const compBasename = getBasename(comp.parsed_info.media_item.file_name);
    // 匹配规则：companion 的 basename 包含某个视频的 basename 或是视频 basename 的子串
    let matched = false;
    for (const [videoBase] of groups) {
      if (compBasename.startsWith(videoBase) || videoBase.startsWith(compBasename) || compBasename === videoBase) {
        const existing = groups.get(videoBase)!;
        existing.push(comp);
        matched = true;
        break;
      }
    }
    if (!matched) {
      // 没有主视频匹配，创建独立 companion 组
      const existing = groups.get(compBasename) || [];
      existing.push(comp);
      groups.set(compBasename, existing);
    }
  }

  return groups;
}

/**
 * 检测是否为剧集组（多 SxxExx / EPxx 文件）
 */
function detectTvGroup(items: RenamePreviewItem[]): boolean {
  const mediaTypes = items.map((i) => i.media_type);
  const tvCount = mediaTypes.filter((t) => t === 'Series' || t === 'Anime').length;
  // 至少2个剧集类型文件 → TV
  return tvCount >= 2;
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

/**
 * 获取文件名（不含扩展名）
 */
function getBasename(fileName: string): string {
  const lastDot = fileName.lastIndexOf('.');
  return lastDot > 0 ? fileName.substring(0, lastDot) : fileName;
}

/**
 * 规范化目录路径（统一分隔符，去尾斜杠）
 */
function normalizeDir(path: string): string {
  return path.replace(/\//g, '\\').replace(/\\+$/, '');
}

/**
 * 从预览列表推断扫描根目录
 */
function detectScanRoot(previews: RenamePreviewItem[]): string {
  if (previews.length === 0) return '';

  // 找出最常见的父目录作为根目录
  const dirCount = new Map<string, number>();
  for (const item of previews) {
    const d = normalizeDir(getParentDir(item.source_path));
    dirCount.set(d, (dirCount.get(d) || 0) + 1);
  }

  let bestDir = '';
  let bestCount = 0;
  for (const [dir, count] of dirCount) {
    if (count > bestCount) {
      bestCount = count;
      bestDir = dir;
    }
  }
  return bestDir;
}

export { aggregateGroupStatus, inferFileRole };
