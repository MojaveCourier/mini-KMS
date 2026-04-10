import { createMutation, createQuery, useQueryClient } from "@tanstack/solid-query";
import { createSignal, For, Show } from "solid-js";

import TableCard from "../components/TableCard";
import { createUser, deleteUser, getUsers, updateUser } from "../lib/api/users";

type FormState = {
  id?: number;
  account: string;
  password: string;
  role: string;
};

const emptyForm: FormState = {
  account: "",
  password: "",
  role: "operator"
};

export default function UsersPage() {
  const queryClient = useQueryClient();
  const [open, setOpen] = createSignal(false);
  const [form, setForm] = createSignal<FormState>(emptyForm);

  const query = createQuery(() => ({
    queryKey: ["users"],
    queryFn: getUsers
  }));

  const refreshAll = async () => {
    await queryClient.invalidateQueries({ queryKey: ["users"] });
    await queryClient.invalidateQueries({ queryKey: ["audit-logs"] });
    await queryClient.invalidateQueries({ queryKey: ["system-status"] });
  };

  const saveMutation = createMutation(() => ({
    mutationFn: async (value: FormState) => {
      if (value.id) {
        return updateUser(value.id, {
          account: value.account,
          password: value.password || undefined,
          role: value.role
        });
      }

      return createUser({
        account: value.account,
        password: value.password,
        role: value.role
      });
    },
    onSuccess: async () => {
      await refreshAll();
      setOpen(false);
      setForm(emptyForm);
    }
  }));

  const deleteMutation = createMutation(() => ({
    mutationFn: deleteUser,
    onSuccess: refreshAll
  }));

  const startCreate = () => {
    setForm(emptyForm);
    setOpen(true);
  };

  const startEdit = (user: { id: number; account: string; role: string }) => {
    setForm({
      id: user.id,
      account: user.account,
      password: "",
      role: user.role
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
        title="User Management"
        action={
          <button class="rounded-2xl bg-ink px-4 py-3 text-sm font-medium text-white" onClick={startCreate}>
            New User
          </button>
        }
      >
        <table class="min-w-full text-left text-sm">
          <thead class="text-slate-500">
            <tr>
              <th class="px-4 py-3">Account</th>
              <th class="px-4 py-3">Role</th>
              <th class="px-4 py-3">Created At</th>
              <th class="px-4 py-3">Actions</th>
            </tr>
          </thead>
          <tbody>
            <For each={query.data ?? []}>
              {(user) => (
                <tr class="border-t border-slate-100">
                  <td class="px-4 py-4 font-medium">{user.account}</td>
                  <td class="px-4 py-4">{user.role}</td>
                  <td class="px-4 py-4 text-slate-500">{new Date(user.created_at).toLocaleString()}</td>
                  <td class="px-4 py-4">
                    <div class="flex gap-2">
                      <button class="rounded-xl bg-slate-900 px-3 py-2 text-white" onClick={() => startEdit(user)}>
                        Edit
                      </button>
                      <button class="rounded-xl bg-coral px-3 py-2 text-white" onClick={() => deleteMutation.mutate(user.id)}>
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
            <h3 class="text-xl font-semibold">{form().id ? "Edit User" : "Create User"}</h3>

            <form class="mt-5 space-y-4" onSubmit={submit}>
              <input
                class="w-full rounded-2xl border border-slate-200 bg-white px-4 py-3"
                placeholder="Account"
                value={form().account}
                onInput={(event) => setForm({ ...form(), account: event.currentTarget.value })}
              />
              <input
                class="w-full rounded-2xl border border-slate-200 bg-white px-4 py-3"
                type="password"
                placeholder={form().id ? "New password (optional)" : "Password"}
                value={form().password}
                onInput={(event) => setForm({ ...form(), password: event.currentTarget.value })}
              />
              <select
                class="w-full rounded-2xl border border-slate-200 bg-white px-4 py-3"
                value={form().role}
                onChange={(event) => setForm({ ...form(), role: event.currentTarget.value })}
              >
                <option value="admin">admin</option>
                <option value="operator">operator</option>
                <option value="viewer">viewer</option>
              </select>

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
