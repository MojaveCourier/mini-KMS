export type User = {
  id: number;
  account: string;
  role: string;
  created_at: string;
};

export type Device = {
  id: number;
  serial: string;
  name: string;
  status: string;
  last_seen_at: string | null;
  created_at: string;
};

export type Keypack = {
  id: number;
  device_id: number;
  device_name: string;
  device_serial: string;
  version: string;
  status: string;
  created_at: string;
};

export type DeviceWritePayload = {
  serial: string;
  name: string;
  status: string;
  last_seen_at: string | null;
};

export type KeypackWritePayload = {
  device_id: number;
  version: string;
  status: string;
};

export type AuditLog = {
  id: number;
  user_id: number | null;
  action: string;
  target_type: string;
  target_id: number | null;
  detail: string | null;
  created_at: string;
};

export type SystemStatus = {
  user_count: number;
  device_count: number;
  active_device_count: number;
  keypack_count: number;
};

export type LoginPayload = {
  account: string;
  password: string;
};

export type LoginResult = {
  token: string;
  user: User;
};

export type SessionUser = User;

export type ApiEnvelope<T> = {
  success: boolean;
  data: T;
};
