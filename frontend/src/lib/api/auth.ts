import type { LoginResult } from "../models";

import { api, unwrap } from "./client";

export function login(account: string, password: string) {
  return unwrap<LoginResult>(
    api.post("login", {
      json: { account, password }
    })
  );
}
