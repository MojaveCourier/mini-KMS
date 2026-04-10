import { api } from "./client";
import type {
  ApiEnvelope,
  AuditLog,
  Device,
  Keypack,
  LoginPayload,
  LoginResult,
  SystemStatus,
  User
} from "../models";

export const kmsApi = {
  login: (payload: LoginPayload) =>
    api.post("login", { json: payload }).json<ApiEnvelope<LoginResult>>(),
  getSystemStatus: () => api.get("system/status").json<ApiEnvelope<SystemStatus>>(),
  getUsers: () => api.get("users").json<ApiEnvelope<User[]>>(),
  createUser: (payload: { account: string; password: string; role: string }) =>
    api.post("users", { json: payload }).json<ApiEnvelope<User>>(),
  updateUser: (id: number, payload: { account: string; password?: string; role: string }) =>
    api.patch(`users/${id}`, { json: payload }).json<ApiEnvelope<User>>(),
  deleteUser: (id: number) =>
    api.delete(`users/${id}`).json<ApiEnvelope<boolean>>(),
  getDevices: () => api.get("devices").json<ApiEnvelope<Device[]>>(),
  getDevice: (id: number) => api.get(`devices/${id}`).json<ApiEnvelope<Device>>(),
  getKeypacks: () => api.get("keypacks").json<ApiEnvelope<Keypack[]>>(),
  getAuditLogs: () => api.get("audit-logs").json<ApiEnvelope<AuditLog[]>>()
};
