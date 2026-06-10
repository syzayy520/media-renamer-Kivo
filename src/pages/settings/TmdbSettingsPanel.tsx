import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { Button, Card, Input } from '../../components/ui';

interface TmdbConfigStatus {
  api_key_configured: boolean;
  gate_enabled: boolean;
  status: string;
  message: string;
}

const statusLabels: Record<string, string> = {
  NotConfigured: '未配置',
  GateDisabled: '已禁用',
  Ready: '待测试',
  Connected: '已连接',
  ConnectionFailed: '连接失败',
};

const statusClasses: Record<string, string> = {
  NotConfigured: 'bg-warning/15 text-warning',
  GateDisabled: 'bg-text-secondary/15 text-text-secondary',
  Ready: 'bg-accent/15 text-accent',
  Connected: 'bg-success/15 text-success',
  ConnectionFailed: 'bg-danger/15 text-danger',
};

export function TmdbSettingsPanel() {
  const [apiKey, setApiKey] = useState('');
  const [configStatus, setConfigStatus] = useState<TmdbConfigStatus | null>(null);
  const [isLoading, setIsLoading] = useState(false);
  const [isTesting, setIsTesting] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [successMessage, setSuccessMessage] = useState<string | null>(null);

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
      const enabled = !configStatus.gate_enabled;
      await invoke('set_tmdb_gate_enabled', { enabled });
      setSuccessMessage(enabled ? 'TMDb Live Search 已启用' : 'TMDb Live Search 已禁用');
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

  const statusLabel = configStatus ? statusLabels[configStatus.status] || configStatus.status : '读取中';
  const statusClass = configStatus ? statusClasses[configStatus.status] || statusClasses.GateDisabled : statusClasses.GateDisabled;
  const canUseGate = Boolean(configStatus?.api_key_configured);
  const canTest = Boolean(configStatus?.api_key_configured && configStatus?.gate_enabled);

  return (
    <Card variant="outlined" padding="lg">
      <div className="flex flex-col gap-4 sm:flex-row sm:items-start sm:justify-between">
        <div className="min-w-0 flex-1">
          <h2 className="text-xl font-semibold text-text-primary">TMDb 配置</h2>
          <p className="mt-2 w-[760px] max-w-full whitespace-normal text-sm leading-6 text-text-secondary">
            配置 TMDb API key 后可启用在线元数据搜索。API key 仅保存在本地，前端只显示配置状态。
          </p>
        </div>
        <span className={`inline-flex shrink-0 items-center rounded-full px-3 py-1 text-xs font-medium ${statusClass}`}>
          {statusLabel}
        </span>
      </div>

      {error && (
        <div className="mt-5 rounded-xl border border-danger/30 bg-danger/10 p-3">
          <p className="text-sm leading-6 text-danger">{error}</p>
        </div>
      )}

      {successMessage && (
        <div className="mt-5 rounded-xl border border-success/30 bg-success/10 p-3">
          <p className="text-sm leading-6 text-success">{successMessage}</p>
        </div>
      )}

      <div className="mt-6 space-y-6">
        <div className="space-y-2">
          <label className="block text-sm font-medium text-text-primary">API Key</label>
          <div className="flex min-w-0 flex-col gap-3 sm:flex-row sm:items-center">
            <Input
              type="password"
              value={apiKey}
              onChange={(event) => setApiKey(event.target.value)}
              placeholder="输入您的 TMDb API key"
              disabled={isLoading}
              className="font-path"
            />
            <Button onClick={handleSaveApiKey} disabled={isLoading || !apiKey.trim()}>
              保存
            </Button>
          </div>
          {configStatus?.api_key_configured && (
            <p className="text-sm text-text-secondary">API key 已配置，输入新 key 可替换。</p>
          )}
        </div>

        <div className="rounded-2xl border border-text-secondary/15 bg-bg-secondary/40 p-4">
          <div className="flex items-center justify-between gap-4">
            <div className="min-w-0 flex-1">
              <h3 className="text-sm font-medium text-text-primary">Live Search</h3>
              <p className="mt-1 w-[680px] max-w-full whitespace-normal text-sm leading-6 text-text-secondary">
                启用后可在预览页使用 TMDb 搜索元数据。
              </p>
            </div>
            <button
              type="button"
              onClick={handleToggleGate}
              disabled={isLoading || !canUseGate}
              className={`relative inline-flex h-7 w-12 shrink-0 items-center rounded-full transition-colors disabled:cursor-not-allowed disabled:opacity-50 ${
                configStatus?.gate_enabled ? 'bg-accent' : 'bg-text-secondary/35'
              }`}
              aria-label="切换 TMDb Live Search"
            >
              <span
                className={`inline-block h-5 w-5 rounded-full bg-white shadow transition-transform ${
                  configStatus?.gate_enabled ? 'translate-x-6' : 'translate-x-1'
                }`}
              />
            </button>
          </div>
          {!canUseGate && (
            <p className="mt-3 text-sm text-warning">请先配置 API key 以启用 Live Search。</p>
          )}
        </div>

        <div className="flex flex-wrap gap-3">
          <Button
            variant="secondary"
            onClick={handleTestConnection}
            disabled={isTesting || !canTest}
          >
            {isTesting ? '测试中...' : '测试连接'}
          </Button>
          <Button
            variant="danger"
            onClick={handleClearApiKey}
            disabled={isLoading || !configStatus?.api_key_configured}
          >
            清除 Key
          </Button>
        </div>

        <div className="rounded-xl border border-accent/25 bg-accent/10 p-4">
          <p className="w-[880px] max-w-full whitespace-normal text-sm leading-6 text-accent">
            安全说明：API key 仅保存在本地，不会上传到任何服务器；前端不会显示完整 key。
          </p>
        </div>
      </div>
    </Card>
  );
}
