import type { Keypack } from "../models";

import { api, unwrap } from "./client";

export function getKeypacks() {
  return unwrap<Keypack[]>(api.get("keypacks"));
}
