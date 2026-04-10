import type { Device } from "../models";

import { api, unwrap } from "./client";

export function getDevices() {
  return unwrap<Device[]>(api.get("devices"));
}

export function getDeviceDetail(id: number) {
  return unwrap<Device>(api.get(`devices/${id}`));
}
