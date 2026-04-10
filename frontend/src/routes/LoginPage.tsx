import { useNavigate } from "@solidjs/router";
import { createSignal } from "solid-js";

import { login } from "../lib/api/auth";
import { writeSession } from "../lib/storage/session";

export default function LoginPage() {
  const navigate = useNavigate();
  const [account, setAccount] = createSignal("admin");
  const [password, setPassword] = createSignal("admin123");
  const [loading, setLoading] = createSignal(false);
  const [error, setError] = createSignal("");

  const submit = async (event: SubmitEvent) => {
    event.preventDefault();
    setLoading(true);
    setError("");

    try {
      const result = await login(account(), password());
      writeSession(result);
      navigate("/dashboard", { replace: true });
    } catch {
      setError("Login failed. Check the account, password, or backend service.");
    } finally {
      setLoading(false);
    }
  };

  return (
    <div class="flex min-h-screen items-center justify-center px-4">
      <form class="card w-full max-w-md p-8" onSubmit={submit}>
        <p class="text-sm uppercase tracking-[0.28em] text-pine">Mini KMS</p>
        <h1 class="mt-3 text-3xl font-semibold text-ink">Admin Login</h1>
        <p class="mt-2 text-sm text-slate-500">Use the default account `admin` / `admin123` to enter the admin flow.</p>

        <label class="mt-8 block text-sm text-slate-600">
          Account
          <input
            class="mt-2 w-full rounded-2xl border border-slate-200 bg-white px-4 py-3 outline-none"
            value={account()}
            onInput={(event) => setAccount(event.currentTarget.value)}
          />
        </label>

        <label class="mt-4 block text-sm text-slate-600">
          Password
          <input
            type="password"
            class="mt-2 w-full rounded-2xl border border-slate-200 bg-white px-4 py-3 outline-none"
            value={password()}
            onInput={(event) => setPassword(event.currentTarget.value)}
          />
        </label>

        <button
          type="submit"
          disabled={loading()}
          class="mt-6 w-full rounded-2xl bg-ink px-4 py-3 font-medium text-white disabled:opacity-60"
        >
          {loading() ? "Signing in..." : "Sign In"}
        </button>

        <p class="mt-4 text-sm text-coral">{error()}</p>
      </form>
    </div>
  );
}
