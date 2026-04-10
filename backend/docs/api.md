# Mini KMS Admin API

基础前缀：`/api`

统一成功返回：

```json
{
  "success": true,
  "data": {}
}
```

统一失败返回：

```json
{
  "success": false,
  "message": "Unauthorized"
}
```

## 公开接口

### `POST /api/login`

请求体：

```json
{
  "account": "admin",
  "password": "admin123"
}
```

## 受保护接口

- `GET /api/system/status`
- `GET /api/users`
- `POST /api/users`
- `PATCH /api/users/:id`
- `DELETE /api/users/:id`
- `GET /api/devices`
- `GET /api/devices/:id`
- `GET /api/keypacks`
- `GET /api/audit-logs`
