import { createQuery } from "@tanstack/solid-query";
import { For } from "solid-js";

import TableCard from "../components/TableCard";
import { getAuditLogs } from "../lib/api/auditLogs";

export default function AuditLogsPage() {
  const query = createQuery(() => ({
    queryKey: ["audit-logs"],
    queryFn: getAuditLogs
  }));

  return (
    <TableCard title="Audit Logs">
      <table class="min-w-full text-left text-sm">
        <thead class="text-slate-500">
          <tr>
            <th class="px-4 py-3">Action</th>
            <th class="px-4 py-3">Target Type</th>
            <th class="px-4 py-3">Target ID</th>
            <th class="px-4 py-3">User ID</th>
            <th class="px-4 py-3">Time</th>
          </tr>
        </thead>
        <tbody>
          <For each={query.data ?? []}>
            {(log) => (
              <tr class="border-t border-slate-100">
                <td class="px-4 py-4">{log.action}</td>
                <td class="px-4 py-4">{log.target_type}</td>
                <td class="px-4 py-4">{log.target_id ?? "-"}</td>
                <td class="px-4 py-4">{log.user_id ?? "-"}</td>
                <td class="px-4 py-4 text-slate-500">{new Date(log.created_at).toLocaleString()}</td>
              </tr>
            )}
          </For>
        </tbody>
      </table>
    </TableCard>
  );
}
