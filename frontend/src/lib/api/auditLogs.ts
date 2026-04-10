import type { AuditLog } from "../models";

import { api, unwrap } from "./client";

export function getAuditLogs() {
  return unwrap<AuditLog[]>(api.get("audit-logs"));
}
