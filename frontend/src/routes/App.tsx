import { Navigate, Route } from "@solidjs/router";
import { Show } from "solid-js";
import { getToken } from "../lib/storage/auth";
import Shell from "../lib/widgets/Shell";
import AuditLogsPage from "./AuditLogsPage";
import DashboardPage from "./DashboardPage";
import DevicesPage from "./DevicesPage";
import KeypacksPage from "./KeypacksPage";
import LoginPage from "./LoginPage";
import UsersPage from "./UsersPage";

function Protected(props: { children: any }) {
  return <Show when={getToken()} fallback={<Navigate href="/login" />}>{props.children}</Show>;
}

export default function App() {
  return (
    <>
      <Route path="/" component={() => <Navigate href="/dashboard" />} />
      <Route path="/login" component={LoginPage} />
      <Route path="/dashboard" component={() => <Protected><Shell><DashboardPage /></Shell></Protected>} />
      <Route path="/users" component={() => <Protected><Shell><UsersPage /></Shell></Protected>} />
      <Route path="/devices" component={() => <Protected><Shell><DevicesPage /></Shell></Protected>} />
      <Route path="/keypacks" component={() => <Protected><Shell><KeypacksPage /></Shell></Protected>} />
      <Route path="/audit-logs" component={() => <Protected><Shell><AuditLogsPage /></Shell></Protected>} />
    </>
  );
}
