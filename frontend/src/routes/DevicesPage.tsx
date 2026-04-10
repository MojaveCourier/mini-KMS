import { A } from "@solidjs/router";
import { createMutation, createQuery, useQueryClient } from "@tanstack/solid-query";
import { createSignal, For, Show } from "solid-js";

import TableCard from "../components/TableCard";
import { createDevice, deleteDevice, getDevices, updateDevice } from "../lib/api/devices";

type FormState = {
  id?: number;
  serial: string;
  name: string;
  status: string;
  last_seen_local: string;
};

const emptyForm: FormState = {
  serial: "",
  name: "",
  status: "active",
  last_seen_local: ""
};

function toDatetimeLocalValue(iso: string | null): string {
  if (!iso) {
    return "";
  }
  const d = new Date(iso);
  if (Number.isNaN(d.getTime())) {
    return "";
  }
  const pad = (n: number) => String(n).padStart(2, "0");
  return `${d.getFullYear()}-${pad(d.getMonth() + 1)}-${pad(d.getDate())}T${pad(d.getHours())}:${pad(d.getMinutes())}`;
}

function fromDatetimeLocalValue(local: string): string | null {
  if (!local.trim()) {
    return null;
  }
  const d = new Date(local);
  return Number.isNaN(d.getTime()) ? null : d.toISOString();
}

export default function DevicesPage() {
  const queryClient = useQueryClient();
  const [open, setOpen] = createSignal(false);
  const [form, setForm] = createSignal<FormState>(emptyForm);

  const query = createQuery(() => ({
    queryKey: ["devices"],
    queryFn: getDevices
  }));

  const refreshAll = async () => {
    await queryClient.invalidateQueries({ queryKey: ["devices"] });
    await queryClient.invalidateQueries({ queryKey: ["audit-logs"] });
    await queryClient.invalidateQueries({ queryKey: ["system-status"] });
    await queryClient.invalidateQueries({ queryKey: ["keypacks"] });
  };

  const saveMutation = createMutation(() => ({
    mutationFn: async (value: FormState) => {
      const payload = {
        serial: value.serial,
        name: value.name,
        status: value.status,
        last_seen_at: fromDatetimeLocalValue(value.last_seen_local)
      };
      if (value.id) {
        return updateDevice(value.id, payload);
      }
      return createDevice(payload);
    },
    onSuccess: async () => {
      await refreshAll();
      setOpen(false);
      setForm(emptyForm);
    }
  }));

  const deleteMutation = createMutation(() => ({
    mutationFn: deleteDevice,
    onSuccess: refreshAll
  }));

  const startCreate = () => {
    setForm(emptyForm);
    setOpen(true);
  };

  const startEdit = (device: { id: number; serial: string; name: string; status: string; last_seen_at: string | null }) => {
    setForm({
      id: device.id,
      serial: device.serial,
      name: device.name,
      status: device.status,
      last_seen_local: toDatetimeLocalValue(device.last_seen_at)
    });
    setOpen(true);
  };

  const submit = async (event: SubmitEvent) => {
    event.preventDefault();
    await saveMutation.mutateAsync(form());
  };

  return (
    <div class="space-y-6">
      <TableCard
        title="Device Management"
        action={
          <button class="rounded-2xl bg-ink px-4 py-3 text-sm font-medium text-white" onClick={startCreate}>
            New Device
          </button>
        }
      >
        <table class="min-w-full text-left text-sm">
          <thead class="text-slate-500">
            <tr>
              <th class="px-4 py-3">Serial</th>
              <th class="px-4 py-3">Name</th>
              <th class="px-4 py-3">Status</th>
              <th class="px-4 py-3">Last Seen</th>
              <th class="px-4 py-3">Detail</th>
              <th class="px-4 py-3">Actions</th>
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
                  <td class="px-4 py-4">
                    <div class="flex flex-wrap gap-2">
                      <button class="rounded-xl bg-slate-900 px-3 py-2 text-white" onClick={() => startEdit(device)}>
                        Edit
                      </button>
                      <button class="rounded-xl bg-coral px-3 py-2 text-white" onClick={() => deleteMutation.mutate(device.id)}>
                        Delete
                      </button>
                    </div>
                  </td>
                </tr>
              )}
            </For>
          </tbody>
        </table>
      </TableCard>

      <Show when={open()}>
        <div class="fixed inset-0 z-50 flex items-center justify-center bg-black/30 p-4">
          <div class="w-full max-w-lg rounded-[28px] border border-slate-200 bg-white p-6 shadow-2xl">
            <h3 class="text-xl font-semibold">{form().id ? "Edit Device" : "Create Device"}</h3>

            <form class="mt-5 space-y-4" onSubmit={submit}>
              <input
                class="w-full rounded-2xl border border-slate-200 bg-white px-4 py-3"
                placeholder="Serial"
                value={form().serial}
                onInput={(event) => setForm({ ...form(), serial: event.currentTarget.value })}
              />
              <input
                class="w-full rounded-2xl border border-slate-200 bg-white px-4 py-3"
                placeholder="Name"
                value={form().name}
                onInput={(event) => setForm({ ...form(), name: event.currentTarget.value })}
              />
              <select
                class="w-full rounded-2xl border border-slate-200 bg-white px-4 py-3"
                value={form().status}
                onChange={(event) => setForm({ ...form(), status: event.currentTarget.value })}
              >
                <option value="active">active</option>
                <option value="inactive">inactive</option>
              </select>
              <div>
                <p class="mb-1 text-xs text-slate-500">Last seen (optional)</p>
                <input
                  class="w-full rounded-2xl border border-slate-200 bg-white px-4 py-3"
                  type="datetime-local"
                  value={form().last_seen_local}
                  onInput={(event) => setForm({ ...form(), last_seen_local: event.currentTarget.value })}
                />
              </div>

              <div class="flex justify-end gap-3">
                <button
                  class="rounded-2xl border border-slate-200 px-4 py-3"
                  type="button"
                  onClick={() => {
                    setOpen(false);
                    setForm(emptyForm);
                  }}
                >
                  Cancel
                </button>
                <button class="rounded-2xl bg-ink px-4 py-3 text-white" type="submit">
                  Save
                </button>
              </div>
            </form>
          </div>
        </div>
      </Show>
    </div>
  );
}
