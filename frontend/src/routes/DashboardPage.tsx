import { createQuery } from "@tanstack/solid-query";

import StatCard from "../components/StatCard";
import { getSystemStatus } from "../lib/api/system";

export default function DashboardPage() {
  const query = createQuery(() => ({
    queryKey: ["system-status"],
    queryFn: getSystemStatus
  }));

  return (
    <section class="space-y-6">
      <div>
        <p class="text-sm uppercase tracking-[0.26em] text-pine">Dashboard</p>
        <h2 class="mt-2 text-3xl font-semibold text-ink">System Status</h2>
        <p class="mt-2 text-sm text-slate-500">Overview of users, devices, active devices, and keypacks.</p>
      </div>

      <div class="grid gap-4 md:grid-cols-2 xl:grid-cols-4">
        <StatCard label="Users" value={query.data?.user_count ?? 0} accent="bg-amber" />
        <StatCard label="Devices" value={query.data?.device_count ?? 0} accent="bg-pine" />
        <StatCard label="Active Devices" value={query.data?.active_device_count ?? 0} accent="bg-coral" />
        <StatCard label="Keypacks" value={query.data?.keypack_count ?? 0} accent="bg-slate-700" />
      </div>
    </section>
  );
}
