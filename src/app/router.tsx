import { Routes, Route } from 'react-router-dom';
import { ScanPage } from '../pages/ScanPage';
import { PreviewPage } from '../pages/preview/PreviewPage';
import { AuditPage } from '../pages/audit/AuditPage';
import { SettingsPage } from '../pages/settings/SettingsPage';

export function AppRoutes() {
  return (
    <Routes>
      <Route path="/" element={<ScanPage />} />
      <Route path="/preview" element={<PreviewPage />} />
      <Route path="/audit" element={<AuditPage />} />
      <Route path="/settings" element={<SettingsPage />} />
    </Routes>
  );
}