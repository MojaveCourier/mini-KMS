import { A } from "@solidjs/router";
import { createQuery } from "@tanstack/solid-query";
import { For } from "solid-js";

import TableCard from "../components/TableCard";
import { getDevices } from "../lib/api/devices";

export default function DevicesPage() {
  const query = createQuery(() => ({
    queryKey: ["devices"],
    queryFn: getDevices
  }));

  return (
    <TableCard title="Device Management">
      <table class="min-w-full text-left text-sm">
        <thead class="text-slate-500">
          <tr>
            <th class="px-4 py-3">Serial</th>
            <th class="px-4 py-3">Name</th>
            <th class="px-4 py-3">Status</th>
            <th class="px-4 py-3">Last Seen</th>
            <th class="px-4 py-3">Detail</th>
          </tr>
        </thead>
        <tbody>
          <For each={query.data ?? []}>
            {(device) => (
              <tr class="border-t border-slate-100">
                <td class="px-4 py-4 font-medium">{device.serial}</td>
                <td class="px-4 py-4">{device.name}</td>
                <td class="px-4 py-4">{device.status}</td>
                <td class="px-4 py-4 text-slate-500">{device.last_seen_at ? new Date(device.last_seen_at).toLocaleString() : "-"}</td>
                <td class="px-4 py-4">
                  <A class="text-pine underline" href={`/devices/${device.id}`}>
                    View
                  </A>
                </td>
              </tr>
            )}
          </For>
        </tbody>
      </table>
    </TableCard>
  );
}
