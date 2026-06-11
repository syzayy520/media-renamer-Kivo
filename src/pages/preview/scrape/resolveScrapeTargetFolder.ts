import type { FolderGroup } from '../../../types';

export function resolveScrapeTargetFolder(group: FolderGroup | null, scanRoot?: string): string {
  if (!group) return '';

  const sourceFolder = resolveCurrentMediaFolder(group);
  if (sourceFolder) return sourceFolder;

  if (scanRoot && isAbsolutePath(scanRoot)) return scanRoot;

  return group.original_path || group.target_path;
}

export function resolvePreparedScrapeTargetFolder(group: FolderGroup | null, scanRoot?: string): string {
  if (!group) return '';

  if (isAbsolutePath(group.target_path)) return group.target_path;

  if (scanRoot && isAbsolutePath(scanRoot) && group.target_path) {
    return joinPath(scanRoot, group.target_path);
  }

  return resolveScrapeTargetFolder(group, scanRoot);
}

function resolveCurrentMediaFolder(group: FolderGroup): string {
  const mainFile = group.children.find((child) => child.file_role === 'MainVideo') || group.children[0];
  if (!mainFile) return '';

  const sourceParent = parentDir(mainFile.original_path);
  if (isAbsolutePath(sourceParent)) return sourceParent;

  if (isAbsolutePath(group.original_path)) return group.original_path;

  return '';
}

function isAbsolutePath(path: string): boolean {
  return /^[a-zA-Z]:[\\/]/.test(path) || path.startsWith('\\\\') || path.startsWith('/');
}

function parentDir(path: string): string {
  const normalized = path.replace(/\\/g, '/');
  const index = normalized.lastIndexOf('/');
  if (index <= 0) return '';
  return path.slice(0, index);
}

function joinPath(parent: string, child: string): string {
  const separator = parent.includes('\\') ? '\\' : '/';
  return `${parent.replace(/[\\/]+$/, '')}${separator}${child.replace(/^[\\/]+/, '')}`;
}
