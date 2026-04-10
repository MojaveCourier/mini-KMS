import { A, useLocation } from "@solidjs/router";
import { Show, type JSX } from "solid-js";

import { clearSession, readSession } from "../lib/storage/session";

const navItems = [
  { href: "/dashboard", label: "系统状态" },
  { href: "/users", label: "用户管理" },
  { href: "/devices", label: "设备管理" },
  { href: "/keypacks", label: "密钥包管理" },
  { href: "/audit-logs", label: "审计日志" }
];

export default function AppShell(props: { children: JSX.Element }) {
  const location = useLocation();
  const session = () => readSession();

  const logout = () => {
    clearSession();
    window.location.href = "/login";
  };

  return (
    <Show when={location.pathname !== "/login"} fallback={props.children}>
      <div class="min-h-screen p-4 md:p-8">
        <div class="mx-auto grid max-w-7xl gap-6 lg:grid-cols-[260px_1fr]">
          <aside class="card p-6">
            <div>
              <p class="text-sm uppercase tracking-[0.24em] text-pine">Mini KMS</p>
              <h1 class="mt-3 text-3xl font-semibold text-ink">Admin Console</h1>
              <p class="mt-3 text-sm text-slate-500">学习前后端主链路的简化后台项目。</p>
            </div>

            <nav class="mt-8 space-y-2">
              {navItems.map((item) => (
                <A
                  href={item.href}
                  class={`block rounded-2xl px-4 py-3 text-sm transition ${
                    location.pathname === item.href
                      ? "bg-ink text-white"
                      : "bg-slate-100/80 text-slate-600 hover:bg-slate-200"
                  }`}
                >
                  {item.label}
                </A>
              ))}
            </nav>

            <div class="mt-8 rounded-2xl bg-mist p-4">
              <p class="text-xs uppercase tracking-[0.2em] text-slate-500">当前登录</p>
              <p class="mt-2 font-medium text-ink">{session()?.user.account}</p>
              <p class="text-sm text-slate-500">{session()?.user.role}</p>
              <button
                class="mt-4 rounded-full bg-coral px-4 py-2 text-sm font-medium text-white"
                onClick={logout}
              >
                退出登录
              </button>
            </div>
          </aside>

          <main>{props.children}</main>
        </div>
      </div>
    </Show>
  );
}
