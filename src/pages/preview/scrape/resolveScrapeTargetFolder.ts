import type { FolderGroup } from '../../../types';

export function resolveScrapeTargetFolder(group: FolderGroup | null): string {
  if (!group) return '';

  if (isAbsolutePath(group.target_path)) {
    return group.target_path;
  }

  const sourceParent = resolveSourceParent(group);
  if (!sourceParent) {
    return group.target_path;
  }

  if (!group.target_path) {
    return sourceParent;
  }

  return joinPath(sourceParent, group.target_path);
}

function resolveSourceParent(group: FolderGroup): string {
  const mainFile = group.children.find((child) => child.file_role === 'MainVideo') || group.children[0];
  if (!mainFile) return '';

  if (isAbsolutePath(group.original_path)) {
    return group.original_path;
  }

  return parentDir(mainFile.original_path);
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
