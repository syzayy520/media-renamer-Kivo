import { Routes, Route } from 'react-router-dom';
import { ScanPage } from '../pages/ScanPage';
import { PreviewPage } from '../pages/preview/PreviewPage';
import { AuditPage } from '../pages/audit/AuditPage';

export function AppRoutes() {
  return (
    <Routes>
      <Route path="/" element={<ScanPage />} />
      <Route path="/preview" element={<PreviewPage />} />
      <Route path="/audit" element={<AuditPage />} />
      <Route path="/settings" element={
        <div className="w-full px-4 py-8">
          <h1 className="text-3xl font-bold text-text-primary">设置页面</h1>
          <p className="text-text-secondary mt-2">设置功能即将上线</p>
        </div>
      } />
    </Routes>
  );
}