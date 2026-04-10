import { createMutation, createQuery, useQueryClient } from "@tanstack/solid-query";
import { createSignal, For, Show } from "solid-js";

import TableCard from "../components/TableCard";
import { getDevices } from "../lib/api/devices";
import { createKeypack, deleteKeypack, getKeypacks, updateKeypack } from "../lib/api/keypacks";

type FormState = {
  id?: number;
  device_id: number;
  version: string;
  status: string;
};

const emptyForm = (): FormState => ({
  device_id: 0,
  version: "",
  status: "draft"
});

export default function KeypacksPage() {
  const queryClient = useQueryClient();
  const [open, setOpen] = createSignal(false);
  const [form, setForm] = createSignal<FormState>(emptyForm());

  const devicesQuery = createQuery(() => ({
    queryKey: ["devices"],
    queryFn: getDevices
  }));

  const keypacksQuery = createQuery(() => ({
    queryKey: ["keypacks"],
    queryFn: getKeypacks
  }));

  const refreshAll = async () => {
    await queryClient.invalidateQueries({ queryKey: ["keypacks"] });
    await queryClient.invalidateQueries({ queryKey: ["audit-logs"] });
    await queryClient.invalidateQueries({ queryKey: ["system-status"] });
  };

  const saveMutation = createMutation(() => ({
    mutationFn: async (value: FormState) => {
      const payload = {
        device_id: value.device_id,
        version: value.version,
        status: value.status
      };
      if (value.id) {
        return updateKeypack(value.id, payload);
      }
      return createKeypack(payload);
    },
    onSuccess: async () => {
      await refreshAll();
      setOpen(false);
      setForm(emptyForm());
    }
  }));

  const deleteMutation = createMutation(() => ({
    mutationFn: deleteKeypack,
    onSuccess: refreshAll
  }));

  const startCreate = () => {
    const list = devicesQuery.data ?? [];
    setForm({
      ...emptyForm(),
      device_id: list[0]?.id ?? 0
    });
    setOpen(true);
  };

  const startEdit = (keypack: { id: number; device_id: number; version: string; status: string }) => {
    setForm({
      id: keypack.id,
      device_id: keypack.device_id,
      version: keypack.version,
      status: keypack.status
    });
    setOpen(true);
  };

  const submit = async (event: SubmitEvent) => {
    event.preventDefault();
    const f = form();
    if (!f.device_id) {
      return;
    }
    await saveMutation.mutateAsync(f);
  };

  return (
    <div class="space-y-6">
      <TableCard
        title="Keypack Management"
        action={
          <button
            class="rounded-2xl bg-ink px-4 py-3 text-sm font-medium text-white"
            onClick={startCreate}
            disabled={!(devicesQuery.data && devicesQuery.data.length > 0)}
          >
            New Keypack
          </button>
        }
      >
        <Show when={devicesQuery.data && devicesQuery.data.length === 0}>
          <p class="text-sm text-slate-500">Create a device first before adding keypacks.</p>
        </Show>
        <table class="min-w-full text-left text-sm">
          <thead class="text-slate-500">
            <tr>
              <th class="px-4 py-3">Version</th>
              <th class="px-4 py-3">Status</th>
              <th class="px-4 py-3">Device</th>
              <th class="px-4 py-3">Created At</th>
              <th class="px-4 py-3">Actions</th>
            </tr>
          </thead>
          <tbody>
            <For each={keypacksQuery.data ?? []}>
              {(keypack) => (
                <tr class="border-t border-slate-100">
                  <td class="px-4 py-4">{keypack.version}</td>
                  <td class="px-4 py-4">{keypack.status}</td>
                  <td class="px-4 py-4">{keypack.device_name}</td>
                  <td class="px-4 py-4 text-slate-500">{new Date(keypack.created_at).toLocaleString()}</td>
                  <td class="px-4 py-4">
                    <div class="flex flex-wrap gap-2">
                      <button class="rounded-xl bg-slate-900 px-3 py-2 text-white" onClick={() => startEdit(keypack)}>
                        Edit
                      </button>
                      <button class="rounded-xl bg-coral px-3 py-2 text-white" onClick={() => deleteMutation.mutate(keypack.id)}>
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
            <h3 class="text-xl font-semibold">{form().id ? "Edit Keypack" : "Create Keypack"}</h3>

            <form class="mt-5 space-y-4" onSubmit={submit}>
              <div>
                <p class="mb-1 text-xs text-slate-500">Device</p>
                <select
                  class="w-full rounded-2xl border border-slate-200 bg-white px-4 py-3"
                  value={String(form().device_id)}
                  onChange={(event) => setForm({ ...form(), device_id: Number(event.currentTarget.value) })}
                >
                  <For each={devicesQuery.data ?? []}>
                    {(device) => <option value={String(device.id)}>{`${device.name} (${device.serial})`}</option>}
                  </For>
                </select>
              </div>
              <input
                class="w-full rounded-2xl border border-slate-200 bg-white px-4 py-3"
                placeholder="Version"
                value={form().version}
                onInput={(event) => setForm({ ...form(), version: event.currentTarget.value })}
              />
              <select
                class="w-full rounded-2xl border border-slate-200 bg-white px-4 py-3"
                value={form().status}
                onChange={(event) => setForm({ ...form(), status: event.currentTarget.value })}
              >
                <option value="draft">draft</option>
                <option value="issued">issued</option>
                <option value="active">active</option>
                <option value="revoked">revoked</option>
              </select>

              <div class="flex justify-end gap-3">
                <button
                  class="rounded-2xl border border-slate-200 px-4 py-3"
                  type="button"
                  onClick={() => {
                    setOpen(false);
                    setForm(emptyForm());
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
