import { Navigate, useLocation } from "@solidjs/router";
import { Show, type JSX } from "solid-js";

import { readSession } from "../lib/storage/session";

export default function ProtectedRoute(props: { children: JSX.Element; fallback: JSX.Element }) {
  const location = useLocation();
  const session = () => readSession();
  const isLoginRoute = () => location.pathname === "/login";

  return (
    <Show
      when={isLoginRoute() ? !session() : !!session()}
      fallback={isLoginRoute() ? <Navigate href="/dashboard" /> : <Navigate href="/login" />}
    >
      {isLoginRoute() ? props.fallback : props.children}
    </Show>
  );
}
