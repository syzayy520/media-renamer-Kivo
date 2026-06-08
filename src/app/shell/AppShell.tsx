// app/shell/AppShell 模块 - 主布局容器
// 职责：应用整体 Shell 布局（Sidebar + TopBar + Content）

import type { ReactNode } from 'react';
import { Sidebar } from './Sidebar';
import { TopBar } from './TopBar';

interface AppShellProps {
  children: ReactNode;
}

export function AppShell({ children }: AppShellProps) {
  return (
    <div className="flex h-screen w-screen overflow-hidden bg-gray-950 text-white">
      <Sidebar />
      <div className="flex min-w-0 flex-1 flex-col">
        <TopBar />
        <main className="min-h-0 flex-1 overflow-auto">
          <div className="mx-auto w-full max-w-[1200px] px-6 py-6">
            {children}
          </div>
        </main>
      </div>
    </div>
  );
}
