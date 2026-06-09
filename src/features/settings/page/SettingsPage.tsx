// features/settings/page/SettingsPage.tsx — 设置页面
// 职责：展示和编辑配置（模板、阈值、API key 状态）

import { useEffect } from 'react';
import { useSettingsPageStore } from '../state/settingsPageStore';
import { toErrorMessage } from '../../../api/invoke';
import { getAllTemplates } from '../../../api/config/getAllTemplates';
import { setTemplate } from '../../../api/config/setTemplate';
import {
  getConfidenceThreshold,
  setConfidenceThreshold as apiSetConfidenceThreshold,
} from '../../../api/config/confidenceThreshold';
import { SettingsErrorPanel } from '../components/SettingsErrorPanel';
import { SettingsSaveStatus } from '../components/SettingsSaveStatus';
import { TemplateList } from '../components/TemplateList';
import { ConfidenceThresholdPanel } from '../components/ConfidenceThresholdPanel';
import { ApiKeyStatusPanel } from '../components/ApiKeyStatusPanel';

export function SettingsPage() {
  const {
    templates,
    confidenceThreshold,
    isLoading,
    error,
    saveStatus,
    saveMessage,
    setTemplates,
    setConfidenceThreshold,
    setIsLoading,
    setError,
    setSaveStatus,
    clearError,
  } = useSettingsPageStore();

  // 加载配置
  useEffect(() => {
    const load = async () => {
      setIsLoading(true);
      try {
        const [tpls, threshold] = await Promise.all([
          getAllTemplates(),
          getConfidenceThreshold(),
        ]);
        setTemplates(tpls);
        setConfidenceThreshold(threshold);
      } catch (err: unknown) {
        setError(toErrorMessage(err));
      }
    };
    load();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  const handleSaveTemplate = async (mediaType: string, template: string) => {
    setSaveStatus('saving');
    try {
      await setTemplate(mediaType, template);
      const updated = await getAllTemplates();
      setTemplates(updated);
      setSaveStatus('saved', `${mediaType} 模板已保存`);
    } catch (err: unknown) {
      setSaveStatus('error', toErrorMessage(err));
    }
  };

  const handleSaveThreshold = async (value: number) => {
    setSaveStatus('saving');
    try {
      await apiSetConfidenceThreshold(value);
      const updated = await getConfidenceThreshold();
      setConfidenceThreshold(updated);
      setSaveStatus('saved', `阈值已更新为 ${updated}`);
    } catch (err: unknown) {
      setSaveStatus('error', toErrorMessage(err));
    }
  };

  return (
    <div className="space-y-4">
      {/* 标题 */}
      <div className="rounded-lg border border-white/10 bg-white/5 p-4">
        <h2 className="mb-1 text-lg font-semibold">设置</h2>
        <p className="text-sm text-white/50">
          当前设置仅影响预览生成，不会真实修改媒体文件。
        </p>
      </div>

      {/* 错误 */}
      {error && <SettingsErrorPanel message={error} onDismiss={clearError} />}

      {/* 保存状态 */}
      <SettingsSaveStatus
        status={saveStatus}
        message={saveMessage}
        onDismiss={() => setSaveStatus('idle')}
      />

      {/* 加载中 */}
      {isLoading && (
        <div className="py-8 text-center text-sm text-white/30">加载配置中...</div>
      )}

      {/* 模板设置 */}
      {!isLoading && (
        <div className="rounded-lg border border-white/10 bg-white/5 p-4">
          <h3 className="mb-3 text-sm font-medium text-white/70">重命名模板</h3>
          <p className="mb-3 text-xs text-white/40">
            修改模板后点击保存，每个模板独立保存。模板变量请参考文档。
          </p>
          <TemplateList templates={templates} onSave={handleSaveTemplate} />
        </div>
      )}

      {/* 置信度阈值 */}
      {!isLoading && (
        <ConfidenceThresholdPanel
          key={String(confidenceThreshold)}
          value={confidenceThreshold}
          onSave={handleSaveThreshold}
        />
      )}

      {/* API Key 状态 */}
      {!isLoading && <ApiKeyStatusPanel />}
    </div>
  );
}
