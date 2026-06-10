import { BrowserRouter as Router } from 'react-router-dom';
import { ErrorBoundary } from '../components/ui';
import { Navigation } from './Navigation';
import { AppRoutes } from './router';

export function AppShell() {
  return (
    <ErrorBoundary>
      <Router>
        <div className="flex h-screen w-screen min-w-0 flex-col overflow-hidden bg-bg-primary text-text-primary">
          <Navigation />
          <main className="min-h-0 flex-1 overflow-auto">
            <AppRoutes />
          </main>
        </div>
      </Router>
    </ErrorBoundary>
  );
}
