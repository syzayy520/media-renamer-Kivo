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
    <nav className="w-full bg-bg-secondary border-b border-text-secondary/20">
      <div className="w-full px-4">
        <div className="flex items-center justify-between h-16">
          <div className="flex items-center shrink-0">
            <Link to="/" className="text-xl font-bold text-accent whitespace-nowrap">
              Media Renamer
            </Link>
          </div>
          
          <div className="flex space-x-4 shrink-0">
            {navItems.map((item) => {
              const Icon = item.icon;
              const isActive = location.pathname === item.path;
              
              return (
                <Link
                  key={item.path}
                  to={item.path}
                  className={`flex items-center px-3 py-2 rounded-md text-sm font-medium transition-colors whitespace-nowrap ${
                    isActive
                      ? 'bg-accent/20 text-accent'
                      : 'text-text-secondary hover:text-text-primary hover:bg-bg-card'
                  }`}
                >
                  <Icon className="w-4 h-4 mr-2" />
                  {item.label}
                </Link>
              );
            })}
          </div>
        </div>
      </div>
    </nav>
  );
}