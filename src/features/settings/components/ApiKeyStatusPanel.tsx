// features/settings/components/ApiKeyStatusPanel.tsx — API Key 安全面板
// 职责：安全展示 TMDb API Key 状态，支持保存/清除，永不显示明文

import { useState, useEffect } from 'react';
import {
  getTmdbApiKeyStatus,
  setTmdbApiKey,
  clearTmdbApiKey,
} from '../../../api/config/tmdbApiKey';
import { Badge } from '../../../shared/ui/Badge';

interface ApiKeyStatusPanelProps {
  onStatusChange?: (status: string) => void;
}

export function ApiKeyStatusPanel({ onStatusChange }: ApiKeyStatusPanelProps) {
  const [configured, setConfigured] = useState<boolean | null>(null);
  const [input, setInput] = useState('');
  const [isSaving, setIsSaving] = useState(false);
  const [isClearing, setIsClearing] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [success, setSuccess] = useState<string | null>(null);

  useEffect(() => {
    const load = async () => {
      try {
        const status = await getTmdbApiKeyStatus();
        setConfigured(status.configured);
      } catch (err: unknown) {
        const msg = err instanceof Error ? err.message : String(err);
        setError(msg);
      }
    };
    load();
  }, []);

  const handleSave = async () => {
    setIsSaving(true);
    setError(null);
    setSuccess(null);
    try {
      const status = await setTmdbApiKey(input);
      setConfigured(status.configured);
      setInput('');
      setSuccess('API Key 已保存');
      onStatusChange?.('saved');
    } catch (err: unknown) {
      const msg = err instanceof Error ? err.message : String(err);
      setError(msg);
    } finally {
      setIsSaving(false);
    }
  };

  const handleClear = async () => {
    setIsClearing(true);
    setError(null);
    setSuccess(null);
    try {
      const status = await clearTmdbApiKey();
      setConfigured(status.configured);
      setInput('');
      setSuccess('API Key 已清除');
      onStatusChange?.('cleared');
    } catch (err: unknown) {
      const msg = err instanceof Error ? err.message : String(err);
      setError(msg);
    } finally {
      setIsClearing(false);
    }
  };

  return (
    <div className="rounded-lg border border-white/10 bg-white/5 p-4">
      <h3 className="text-sm font-medium text-white/70">TMDb API Key</h3>
      <p className="mt-0.5 mb-3 text-xs text-white/40">
        API Key 会被安全保存，界面不会显示明文。当前功能只保存配置，不会联网请求 TMDb。
      </p>

      {/* 状态 */}
      <div className="mb-4 flex items-center gap-3">
        {configured === null ? (
          <Badge variant="info">加载中...</Badge>
        ) : configured ? (
          <Badge variant="success">Configured</Badge>
        ) : (
          <Badge variant="warning">Not Configured</Badge>
        )}
        <span className="text-xs text-white/30">不显示明文 key</span>
      </div>

      {/* 输入 + 保存 */}
      <div className="mb-3 flex gap-2">
        <input
          type="password"
          value={input}
          onChange={(e) => setInput(e.target.value)}
          placeholder="输入新的 TMDb API Key"
          disabled={isSaving}
          className="min-w-0 flex-1 rounded border border-white/10 bg-white/5 px-3 py-1.5 text-sm text-white placeholder-white/30 outline-none transition-colors focus:border-blue-500/50"
        />
        <button
          type="button"
          disabled={!input.trim() || isSaving}
          onClick={handleSave}
          className={`shrink-0 rounded px-4 py-1.5 text-xs font-medium transition-colors ${
            !input.trim() || isSaving
              ? 'cursor-not-allowed bg-white/5 text-white/20'
              : 'bg-blue-600 text-white hover:bg-blue-500'
          }`}
        >
          {isSaving ? '保存中...' : '保存 API Key'}
        </button>
        <button
          type="button"
          disabled={isClearing || !configured}
          onClick={handleClear}
          className={`shrink-0 rounded px-4 py-1.5 text-xs font-medium transition-colors ${
            isClearing || !configured
              ? 'cursor-not-allowed bg-white/5 text-white/20'
              : 'bg-red-600/20 text-red-400 hover:bg-red-600/30'
          }`}
        >
          {isClearing ? '清除中...' : '清除'}
        </button>
      </div>

      {/* 错误 */}
      {error && (
        <div className="rounded border border-red-500/20 bg-red-500/5 px-3 py-2">
          <p className="text-xs text-red-400">{error}</p>
        </div>
      )}

      {/* 成功 */}
      {success && (
        <div className="rounded border border-emerald-500/20 bg-emerald-500/5 px-3 py-2">
          <p className="text-xs text-emerald-400">{success}</p>
        </div>
      )}
    </div>
  );
}
