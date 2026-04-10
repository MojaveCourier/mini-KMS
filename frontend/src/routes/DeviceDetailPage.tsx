import { useParams } from "@solidjs/router";
import { createQuery } from "@tanstack/solid-query";

import TableCard from "../components/TableCard";
import { getDeviceDetail } from "../lib/api/devices";

export default function DeviceDetailPage() {
  const params = useParams();
  const query = createQuery(() => ({
    queryKey: ["device", params.id],
    queryFn: () => getDeviceDetail(Number(params.id))
  }));

  return (
    <TableCard title="Device Detail">
      <div class="grid gap-4 md:grid-cols-2">
        <div>
          <p class="text-sm text-slate-500">Serial</p>
          <p class="mt-1 text-lg font-medium">{query.data?.serial}</p>
        </div>
        <div>
          <p class="text-sm text-slate-500">Name</p>
          <p class="mt-1 text-lg font-medium">{query.data?.name}</p>
        </div>
        <div>
          <p class="text-sm text-slate-500">Status</p>
          <p class="mt-1 text-lg font-medium">{query.data?.status}</p>
        </div>
        <div>
          <p class="text-sm text-slate-500">Last Seen</p>
          <p class="mt-1 text-lg font-medium">
            {query.data?.last_seen_at ? new Date(query.data.last_seen_at).toLocaleString() : "-"}
          </p>
        </div>
        <div>
          <p class="text-sm text-slate-500">Created At</p>
          <p class="mt-1 text-lg font-medium">
            {query.data?.created_at ? new Date(query.data.created_at).toLocaleString() : "-"}
          </p>
        </div>
      </div>
    </TableCard>
  );
}
