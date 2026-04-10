import type { SystemStatus } from "../models";

import { api, unwrap } from "./client";

export function getSystemStatus() {
  return unwrap<SystemStatus>(api.get("system/status"));
}
