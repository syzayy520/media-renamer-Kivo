import type { FolderGroup } from '../../../../types';

export function buildTmdbQuery(group: FolderGroup): string {
  const mainVideo = group.children.find((child) => child.file_role === 'MainVideo');
  const rawName = group.target_folder_name || mainVideo?.target_name || group.original_folder_name;
  const normalized = rawName
    .replace(/\.[a-z0-9]{2,5}$/i, '')
    .replace(/^\.+/, '')
    .replace(/[._]+/g, ' ')
    .replace(/\[[^\]]*\]/g, ' ')
    .replace(/\s+/g, ' ')
    .trim();
  const beforeYear = normalized.match(/^(.*?)(?:\s*\(?((?:19|20)\d{2})\)?)(?:\s|$)/)?.[1]?.trim();

  if (beforeYear) return beforeYear;

  return normalized
    .replace(/\b(720p|1080p|2160p|4k|bluray|blu ray|web dl|web-dl|webrip|hdtv|remux|unrated|proper|repack|extended|gb|gbr|usa|x264|x265|hevc|h264|h265|vc1|vc|10bit|8bit|hdr|dv|truehd|dts|atmos)\b/gi, ' ')
    .replace(/\s+/g, ' ')
    .trim();
}
