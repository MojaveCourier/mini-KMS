import type { Device, DeviceWritePayload } from "../models";

import { api, unwrap } from "./client";

export function getDevices() {
  return unwrap<Device[]>(api.get("devices"));
}

export function getDeviceDetail(id: number) {
  return unwrap<Device>(api.get(`devices/${id}`));
}

export function createDevice(payload: DeviceWritePayload) {
  return unwrap<Device>(api.post("devices", { json: payload }));
}

export function updateDevice(id: number, payload: DeviceWritePayload) {
  return unwrap<Device>(api.patch(`devices/${id}`, { json: payload }));
}

export function deleteDevice(id: number) {
  return unwrap<boolean>(api.delete(`devices/${id}`));
}
