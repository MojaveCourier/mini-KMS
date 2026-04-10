import type { User } from "../models";

import { api, unwrap } from "./client";

export function getUsers() {
  return unwrap<User[]>(api.get("users"));
}

export function createUser(payload: { account: string; password: string; role: string }) {
  return unwrap<User>(api.post("users", { json: payload }));
}

export function updateUser(id: number, payload: { account?: string; password?: string; role?: string }) {
  return unwrap<User>(api.patch(`users/${id}`, { json: payload }));
}

export function deleteUser(id: number) {
  return unwrap<boolean>(api.delete(`users/${id}`));
}
