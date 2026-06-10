import { TmdbSettingsPanel } from './TmdbSettingsPanel';

export function SettingsPage() {
  return (
    <div className="w-full px-4 py-8">
      <h1 className="text-3xl font-bold text-text-primary mb-6">设置</h1>
      
      <div className="space-y-6">
        {/* TMDb Configuration */}
        <TmdbSettingsPanel />

        {/* Future settings sections can be added here */}
      </div>
    </div>
  );
}
