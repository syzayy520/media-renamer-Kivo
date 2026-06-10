import type { FolderGroup } from '../../../types';

export function resolveScrapeTargetFolder(group: FolderGroup | null, scanRoot?: string): string {
  if (!group) return '';

  if (isAbsolutePath(group.target_path)) {
    return group.target_path;
  }

  const absoluteBase = resolveAbsoluteBase(group, scanRoot);
  if (!absoluteBase) {
    return group.target_path;
  }

  if (!group.target_path) {
    return absoluteBase;
  }

  return joinPath(absoluteBase, group.target_path);
}

function resolveAbsoluteBase(group: FolderGroup, scanRoot?: string): string {
  if (scanRoot && isAbsolutePath(scanRoot)) {
    return scanRoot;
  }

  if (isAbsolutePath(group.original_path)) {
    return group.original_path;
  }

  const mainFile = group.children.find((child) => child.file_role === 'MainVideo') || group.children[0];
  if (!mainFile) return '';

  const sourceParent = parentDir(mainFile.original_path);
  if (isAbsolutePath(sourceParent)) {
    return sourceParent;
  }

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
