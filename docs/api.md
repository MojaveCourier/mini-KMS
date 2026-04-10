# API List

## Response Format

Success:

```json
{
  "success": true,
  "data": {}
}
```

Failure:

```json
{
  "success": false,
  "message": "error message"
}
```

## Auth

- Protected endpoints require `Authorization: Bearer <token>`

## Endpoints

- `POST /api/login`
- `GET /api/users`
- `POST /api/users`
- `PATCH /api/users/:id`
- `DELETE /api/users/:id`
- `GET /api/devices`
- `POST /api/devices`
- `GET /api/devices/:id`
- `PATCH /api/devices/:id`
- `DELETE /api/devices/:id`
- `GET /api/keypacks`
- `POST /api/keypacks`
- `PATCH /api/keypacks/:id`
- `DELETE /api/keypacks/:id`
- `GET /api/system/status`
- `GET /api/audit-logs`
