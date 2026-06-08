import { BrowserRouter as Router } from 'react-router-dom';
import { ErrorBoundary } from '../components/ui';
import { Navigation } from './Navigation';
import { AppRoutes } from './router';

export function AppShell() {
  return (
    <ErrorBoundary>
      <Router>
        <div className="min-h-screen bg-bg-primary">
          <Navigation />
          <main>
            <AppRoutes />
          </main>
        </div>
      </Router>
    </ErrorBoundary>
  );
}