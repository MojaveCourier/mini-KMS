import type { SessionUser } from "../models";

const SESSION_KEY = "mini-kms-admin-session";

export type Session = {
  token: string;
  user: SessionUser;
};

export function readSession(): Session | null {
  const raw = localStorage.getItem(SESSION_KEY);
  return raw ? (JSON.parse(raw) as Session) : null;
}

export function writeSession(session: Session) {
  localStorage.setItem(SESSION_KEY, JSON.stringify(session));
}

export function clearSession() {
  localStorage.removeItem(SESSION_KEY);
}
