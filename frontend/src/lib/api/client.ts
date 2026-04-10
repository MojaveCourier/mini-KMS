import ky from "ky";

import { clearSession, readSession } from "../storage/session";

export const api = ky.create({
  prefixUrl: "http://127.0.0.1:3000/api",
  hooks: {
    beforeRequest: [
      (request) => {
        const session = readSession();
        if (session?.token) {
          request.headers.set("Authorization", `Bearer ${session.token}`);
        }
      }
    ],
    afterResponse: [
      async (_request, _options, response) => {
        if (response.status === 401) {
          clearSession();
          if (window.location.pathname !== "/login") {
            window.location.href = "/login";
          }
        }
      }
    ]
  }
});

export async function unwrap<T>(request: Promise<Response>) {
  const response = await request;
  const body = (await response.json()) as { success: boolean; data: T };
  return body.data;
}
