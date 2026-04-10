import type { Keypack, KeypackWritePayload } from "../models";

import { api, unwrap } from "./client";

export function getKeypacks() {
  return unwrap<Keypack[]>(api.get("keypacks"));
}

export function createKeypack(payload: KeypackWritePayload) {
  return unwrap<Keypack>(api.post("keypacks", { json: payload }));
}

export function updateKeypack(id: number, payload: KeypackWritePayload) {
  return unwrap<Keypack>(api.patch(`keypacks/${id}`, { json: payload }));
}

export function deleteKeypack(id: number) {
  return unwrap<boolean>(api.delete(`keypacks/${id}`));
}
