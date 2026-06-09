// api/config/tmdbApiKey.ts — TMDb API Key 命令封装
// 职责：get_tmdb_api_key_status / set_tmdb_api_key / clear_tmdb_api_key

import { invokeCommand } from '../invoke';

export interface TmdbApiKeyStatus {
  configured: boolean;
}

export async function getTmdbApiKeyStatus(): Promise<TmdbApiKeyStatus> {
  return invokeCommand<TmdbApiKeyStatus>('get_tmdb_api_key_status');
}

export async function setTmdbApiKey(apiKey: string): Promise<TmdbApiKeyStatus> {
  return invokeCommand<TmdbApiKeyStatus>('set_tmdb_api_key', { apiKey });
}

export async function clearTmdbApiKey(): Promise<TmdbApiKeyStatus> {
  return invokeCommand<TmdbApiKeyStatus>('clear_tmdb_api_key');
}
