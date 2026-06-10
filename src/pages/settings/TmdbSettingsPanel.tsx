import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';

interface TmdbConfigStatus {
  api_key_configured: boolean;
  gate_enabled: boolean;
  status: string;
  message: string;
}

export function TmdbSettingsPanel() {
  const [apiKey, setApiKey] = useState('');
  const [configStatus, setConfigStatus] = useState<TmdbConfigStatus | null>(null);
  const [isLoading, setIsLoading] = useState(false);
  const [isTesting, setIsTesting] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [successMessage, setSuccessMessage] = useState<string | null>(null);

  // Fetch current status on mount
  useEffect(() => {
    fetchConfigStatus();
  }, []);

  const fetchConfigStatus = async () => {
    try {
      setIsLoading(true);
      setError(null);
      const status = await invoke<TmdbConfigStatus>('get_tmdb_config_status');
      setConfigStatus(status);
    } catch (err) {
      setError(err as string);
    } finally {
      setIsLoading(false);
    }
  };

  const handleSaveApiKey = async () => {
    if (!apiKey.trim()) {
      setError('API key 不能为空');
      return;
    }

    try {
      setIsLoading(true);
      setError(null);
      setSuccessMessage(null);

      await invoke('set_tmdb_api_key', { apiKey: apiKey.trim() });
      setApiKey('');
      setSuccessMessage('API key 已保存');
      await fetchConfigStatus();
    } catch (err) {
      setError(err as string);
    } finally {
      setIsLoading(false);
    }
  };

  const handleClearApiKey = async () => {
    try {
      setIsLoading(true);
      setError(null);
      setSuccessMessage(null);

      await invoke('clear_tmdb_api_key');
      setSuccessMessage('API key 已清除');
      await fetchConfigStatus();
    } catch (err) {
      setError(err as string);
    } finally {
      setIsLoading(false);
    }
  };

  const handleToggleGate = async () => {
    if (!configStatus) return;

    try {
      setIsLoading(true);
      setError(null);
      setSuccessMessage(null);

      const newEnabled = !configStatus.gate_enabled;
      await invoke('set_tmdb_gate_enabled', { enabled: newEnabled });
      setSuccessMessage(newEnabled ? 'TMDb live search 已启用' : 'TMDb live search 已禁用');
      await fetchConfigStatus();
    } catch (err) {
      setError(err as string);
    } finally {
      setIsLoading(false);
    }
  };

  const handleTestConnection = async () => {
    try {
      setIsTesting(true);
      setError(null);
      setSuccessMessage(null);

      const result = await invoke<TmdbConfigStatus>('test_tmdb_connection');
      setConfigStatus(result);

      if (result.status === 'Connected') {
        setSuccessMessage(result.message);
      } else {
        setError(result.message);
      }
    } catch (err) {
      setError(err as string);
    } finally {
      setIsTesting(false);
    }
  };

  const getStatusBadge = () => {
    if (!configStatus) return null;

    const statusColors: Record<string, string> = {
      NotConfigured: 'bg-yellow-100 text-yellow-800',
      GateDisabled: 'bg-gray-100 text-gray-800',
      Ready: 'bg-blue-100 text-blue-800',
      Connected: 'bg-green-100 text-green-800',
      ConnectionFailed: 'bg-red-100 text-red-800',
    };

    const statusLabels: Record<string, string> = {
      NotConfigured: '未配置',
      GateDisabled: '已禁用',
      Ready: '待测试',
      Connected: '已连接',
      ConnectionFailed: '连接失败',
    };

    return (
      <span className={`px-2 py-1 rounded-full text-xs font-medium ${statusColors[configStatus.status] || 'bg-gray-100 text-gray-800'}`}>
        {statusLabels[configStatus.status] || configStatus.status}
      </span>
    );
  };

  return (
    <div className="bg-surface rounded-lg border border-border p-6">
      <div className="flex items-center justify-between mb-4">
        <h2 className="text-lg font-semibold text-text-primary">TMDb 配置</h2>
        {getStatusBadge()}
      </div>

      <p className="text-text-secondary text-sm mb-6">
        配置 TMDb API key 以启用元数据搜索功能。
        <a
          href="https://www.themoviedb.org/settings/api"
          target="_blank"
          rel="noopener noreferrer"
          className="text-primary hover:underline ml-1"
        >
          获取 API key
        </a>
      </p>

      {/* Error Message */}
      {error && (
        <div className="mb-4 p-3 bg-red-50 border border-red-200 rounded-md">
          <p className="text-red-800 text-sm">{error}</p>
        </div>
      )}

      {/* Success Message */}
      {successMessage && (
        <div className="mb-4 p-3 bg-green-50 border border-green-200 rounded-md">
          <p className="text-green-800 text-sm">{successMessage}</p>
        </div>
      )}

      {/* API Key Input */}
      <div className="mb-6">
        <label className="block text-sm font-medium text-text-primary mb-2">
          API Key
        </label>
        <div className="flex gap-2">
          <input
            type="password"
            value={apiKey}
            onChange={(e) => setApiKey(e.target.value)}
            placeholder="输入您的 TMDb API key"
            className="flex-1 px-3 py-2 border border-border rounded-md bg-background text-text-primary focus:outline-none focus:ring-2 focus:ring-primary"
            disabled={isLoading}
          />
          <button
            onClick={handleSaveApiKey}
            disabled={isLoading || !apiKey.trim()}
            className="px-4 py-2 bg-primary text-white rounded-md hover:bg-primary/90 disabled:opacity-50 disabled:cursor-not-allowed"
          >
            保存
          </button>
        </div>
        {configStatus?.api_key_configured && (
          <p className="mt-2 text-sm text-text-secondary">
            API key 已配置。输入新 key 可替换。
          </p>
        )}
      </div>

      {/* Gate Control */}
      <div className="mb-6">
        <div className="flex items-center justify-between">
          <div>
            <h3 className="text-sm font-medium text-text-primary">Live Search</h3>
            <p className="text-sm text-text-secondary">
              启用后可使用 TMDb 搜索元数据
            </p>
          </div>
          <button
            onClick={handleToggleGate}
            disabled={isLoading || !configStatus?.api_key_configured}
            className={`relative inline-flex h-6 w-11 items-center rounded-full transition-colors ${
              configStatus?.gate_enabled ? 'bg-primary' : 'bg-gray-200'
            } ${!configStatus?.api_key_configured ? 'opacity-50 cursor-not-allowed' : ''}`}
          >
            <span
              className={`inline-block h-4 w-4 transform rounded-full bg-white transition-transform ${
                configStatus?.gate_enabled ? 'translate-x-6' : 'translate-x-1'
              }`}
            />
          </button>
        </div>
        {!configStatus?.api_key_configured && (
          <p className="mt-2 text-sm text-yellow-600">
            请先配置 API key 以启用 live search
          </p>
        )}
      </div>

      {/* Action Buttons */}
      <div className="flex gap-2">
        <button
          onClick={handleTestConnection}
          disabled={isTesting || !configStatus?.api_key_configured || !configStatus?.gate_enabled}
          className="px-4 py-2 border border-border rounded-md text-text-primary hover:bg-surface-hover disabled:opacity-50 disabled:cursor-not-allowed"
        >
          {isTesting ? '测试中...' : '测试连接'}
        </button>
        <button
          onClick={handleClearApiKey}
          disabled={isLoading || !configStatus?.api_key_configured}
          className="px-4 py-2 border border-red-200 text-red-600 rounded-md hover:bg-red-50 disabled:opacity-50 disabled:cursor-not-allowed"
        >
          清除 Key
        </button>
      </div>

      {/* Security Notice */}
      <div className="mt-6 p-3 bg-blue-50 border border-blue-200 rounded-md">
        <p className="text-blue-800 text-sm">
          <strong>安全说明：</strong>API key 仅保存在本地，不会上传到任何服务器。
          前端仅显示配置状态，不会显示完整 key。
        </p>
      </div>
    </div>
  );
}
