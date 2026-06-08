import { BrowserRouter as Router } from 'react-router-dom';
import { ErrorBoundary } from '../components/ui';
import { Navigation } from './Navigation';
import { AppRoutes } from './router';

export function AppShell() {
  return (
    <ErrorBoundary>
      <Router>
        <div className="w-full min-h-screen flex flex-col bg-bg-primary">
          <Navigation />
          <main className="flex-1 min-w-0 overflow-auto">
            <AppRoutes />
          </main>
        </div>
      </Router>
    </ErrorBoundary>
  );
}