// app/App 模块 - App root composition
// 职责：组合 Router + AppShell，不承担路由定义和布局实现

import { BrowserRouter } from 'react-router-dom';
import { AppShell } from './shell/AppShell';
import { AppRouter } from './router';

export function App() {
  return (
    <BrowserRouter>
      <AppShell>
        <AppRouter />
      </AppShell>
    </BrowserRouter>
  );
}
