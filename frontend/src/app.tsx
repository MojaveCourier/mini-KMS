import { QueryClient, QueryClientProvider } from "@tanstack/solid-query";
import { Navigate, Route, Router } from "@solidjs/router";

import AppShell from "./components/AppShell";
import ProtectedRoute from "./components/ProtectedRoute";
import AuditLogsPage from "./routes/AuditLogsPage";
import DashboardPage from "./routes/DashboardPage";
import DeviceDetailPage from "./routes/DeviceDetailPage";
import DevicesPage from "./routes/DevicesPage";
import KeypacksPage from "./routes/KeypacksPage";
import LoginPage from "./routes/LoginPage";
import UsersPage from "./routes/UsersPage";

const queryClient = new QueryClient();

export default function App() {
  return (
    <QueryClientProvider client={queryClient}>
      <Router
        root={(props) => (
          <ProtectedRoute fallback={<LoginPage />}>
            <AppShell>{props.children}</AppShell>
          </ProtectedRoute>
        )}
      >
        <Route path="/login" component={LoginPage} />
        <Route path="/" component={() => <Navigate href="/dashboard" />} />
        <Route path="/dashboard" component={DashboardPage} />
        <Route path="/users" component={UsersPage} />
        <Route path="/devices" component={DevicesPage} />
        <Route path="/devices/:id" component={DeviceDetailPage} />
        <Route path="/keypacks" component={KeypacksPage} />
        <Route path="/audit-logs" component={AuditLogsPage} />
      </Router>
    </QueryClientProvider>
  );
}
