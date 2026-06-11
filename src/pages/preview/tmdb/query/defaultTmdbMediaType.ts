import type { FolderGroup } from '../../../../types';
import type { TmdbManualMediaType } from '../tmdbInspectorTypes';

export function defaultTmdbMediaType(group: FolderGroup | null): TmdbManualMediaType {
  return group?.media_type === 'Tv' ? 'tv' : 'movie';
}
