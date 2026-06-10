import { Link, useLocation } from 'react-router-dom';
import { Home, Eye, Shield, Settings } from 'lucide-react';

export function Navigation() {
  const location = useLocation();
  
  const navItems = [
    { path: '/', label: '扫描', icon: Home },
    { path: '/preview', label: '预览', icon: Eye },
    { path: '/audit', label: '审核', icon: Shield },
    { path: '/settings', label: '设置', icon: Settings },
  ];

  return (
    <nav className="w-full shrink-0 border-b border-text-secondary/15 bg-bg-secondary">
      <div className="mx-auto flex h-16 w-full max-w-7xl items-center justify-between px-4 sm:px-6">
        <Link to="/" className="shrink-0 text-xl font-bold text-accent whitespace-nowrap">
          Media Renamer
        </Link>
        
        <div className="flex shrink-0 items-center gap-2">
          {navItems.map((item) => {
            const Icon = item.icon;
            const isActive = location.pathname === item.path;
            
            return (
              <Link
                key={item.path}
                to={item.path}
                className={`inline-flex items-center rounded-xl px-3 py-2 text-sm font-medium transition-colors whitespace-nowrap ${
                  isActive
                    ? 'bg-accent/20 text-accent'
                    : 'text-text-secondary hover:bg-bg-card hover:text-text-primary'
                }`}
              >
                <Icon className="mr-2 h-4 w-4 shrink-0" />
                {item.label}
              </Link>
            );
          })}
        </div>
      </div>
    </nav>
  );
}
