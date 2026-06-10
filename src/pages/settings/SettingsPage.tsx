import { TmdbSettingsPanel } from './TmdbSettingsPanel';

export function SettingsPage() {
  return (
    <div className="w-full px-6 py-8">
      <div className="mx-auto w-full max-w-6xl space-y-6">
        <div className="space-y-2">
          <h1 className="text-3xl font-bold leading-tight text-text-primary">设置</h1>
          <p className="text-sm leading-6 text-text-secondary break-keep">
            管理 TMDb 搜索、API key 和功能开关。
          </p>
        </div>

        <TmdbSettingsPanel />
      </div>
    </div>
  );
}
