import { A, useLocation, useNavigate } from "@solidjs/router";
import type { JSX } from "solid-js";
import { clearSession, getCurrentUser } from "../storage/auth";

const navItems = [
  { href: "/dashboard", label: "Dashboard" },
  { href: "/users", label: "Users" },
  { href: "/devices", label: "Devices" },
  { href: "/keypacks", label: "Keypacks" },
  { href: "/audit-logs", label: "Audit Logs" }
];

export default function Shell(props: { children: JSX.Element }) {
  const location = useLocation();
  const navigate = useNavigate();
  const user = getCurrentUser();

  return (
    <div class="min-h-screen p-4 md:p-6">
      <div class="mx-auto grid min-h-[calc(100vh-2rem)] max-w-7xl gap-4 md:grid-cols-[240px_1fr]">
        <aside class="rounded-[28px] border border-[var(--line)] bg-[var(--panel)] p-5 shadow-[0_20px_60px_rgba(90,63,40,0.08)] backdrop-blur">
          <div class="mb-8">
            <div class="text-xs uppercase tracking-[0.3em] text-[var(--muted)]">Mini KMS</div>
            <h1 class="mt-2 text-2xl font-semibold">Admin</h1>
            <p class="mt-3 text-sm text-[var(--muted)]">Learning admin for KMS basics</p>
          </div>

          <nav class="space-y-2">
            {navItems.map((item) => (
              <A
                href={item.href}
                class={`block rounded-2xl px-4 py-3 text-sm transition ${
                  location.pathname === item.href
                    ? "bg-[var(--accent)] text-white shadow-lg"
                    : "text-[var(--ink)] hover:bg-[var(--accent-soft)]"
                }`}
              >
                {item.label}
              </A>
            ))}
          </nav>

          <div class="mt-8 rounded-2xl border border-[var(--line)] bg-white/70 p-4">
            <div class="text-xs text-[var(--muted)]">Signed In As</div>
            <div class="mt-2 font-medium">{user?.account}</div>
            <div class="text-sm text-[var(--muted)]">{user?.role}</div>
            <button
              class="mt-4 w-full rounded-xl bg-[var(--ink)] px-3 py-2 text-sm text-white"
              onClick={() => {
                clearSession();
                navigate("/login", { replace: true });
              }}
            >
              Sign Out
            </button>
          </div>
        </aside>

        <main class="rounded-[28px] border border-[var(--line)] bg-[var(--panel)] p-5 shadow-[0_20px_60px_rgba(90,63,40,0.08)] backdrop-blur md:p-6">
          {props.children}
        </main>
      </div>
    </div>
  );
}
