import { createQuery } from "@tanstack/solid-query";
import { For } from "solid-js";

import TableCard from "../components/TableCard";
import { getKeypacks } from "../lib/api/keypacks";

export default function KeypacksPage() {
  const query = createQuery(() => ({
    queryKey: ["keypacks"],
    queryFn: getKeypacks
  }));

  return (
    <TableCard title="Keypack Management">
      <table class="min-w-full text-left text-sm">
        <thead class="text-slate-500">
          <tr>
            <th class="px-4 py-3">Version</th>
            <th class="px-4 py-3">Status</th>
            <th class="px-4 py-3">Device</th>
            <th class="px-4 py-3">Created At</th>
          </tr>
        </thead>
        <tbody>
          <For each={query.data ?? []}>
            {(keypack) => (
              <tr class="border-t border-slate-100">
                <td class="px-4 py-4">{keypack.version}</td>
                <td class="px-4 py-4">{keypack.status}</td>
                <td class="px-4 py-4">{keypack.device_name}</td>
                <td class="px-4 py-4 text-slate-500">{new Date(keypack.created_at).toLocaleString()}</td>
              </tr>
            )}
          </For>
        </tbody>
      </table>
    </TableCard>
  );
}
