// app/router 模块 - 路由注册
// 职责：定义所有前端路由，不承担布局逻辑

import { Routes, Route } from 'react-router-dom';
import { ScanPage } from '../features/scan/page/ScanPage';
import { PreviewPage } from '../features/preview/page/PreviewPage';
import { SafetyPage } from '../features/safety/page/SafetyPage';
import { AuditPage } from '../features/audit/page/AuditPage';
import { SettingsPage } from '../features/settings/page/SettingsPage';

export function AppRouter() {
  return (
    <Routes>
      <Route path="/" element={<ScanPage />} />
      <Route path="/preview" element={<PreviewPage />} />
      <Route path="/safety" element={<SafetyPage />} />
      <Route path="/audit" element={<AuditPage />} />
      <Route path="/settings" element={<SettingsPage />} />
    </Routes>
  );
}
