// app/shell/Sidebar 模块 - 侧边栏组件
// 职责：左侧导航栏，显示页面路由链接

import { NavLink } from 'react-router-dom';

interface NavItem {
  path: string;
  label: string;
}

const navItems: NavItem[] = [
  { path: '/', label: '扫描' },
  { path: '/preview', label: '预览' },
  { path: '/safety', label: '安全检查' },
  { path: '/audit', label: '审计' },
  { path: '/settings', label: '设置' },
];

export function Sidebar() {
  return (
    <aside className="flex h-full w-[220px] shrink-0 flex-col border-r border-white/10 bg-black/40">
      {/* Logo */}
      <div className="flex h-14 items-center gap-2 border-b border-white/10 px-4">
        <div className="flex h-7 w-7 items-center justify-center rounded bg-blue-600 text-xs font-bold text-white">
          M
        </div>
        <span className="text-sm font-semibold">Media Renamer Kivo</span>
      </div>

      {/* Nav */}
      <nav className="flex-1 overflow-y-auto px-2 py-3">
        {navItems.map((item) => (
          <NavLink
            key={item.path}
            to={item.path}
            end={item.path === '/'}
            className={({ isActive }) =>
              `mb-0.5 flex items-center rounded-md px-3 py-2 text-sm transition-colors ${
                isActive
                  ? 'bg-blue-600/20 text-blue-400'
                  : 'text-white/60 hover:bg-white/5 hover:text-white/80'
              }`
            }
          >
            {item.label}
          </NavLink>
        ))}
      </nav>

      {/* Footer */}
      <div className="border-t border-white/10 px-4 py-3">
        <p className="text-[11px] text-white/40">v0.1.0</p>
      </div>
    </aside>
  );
}
