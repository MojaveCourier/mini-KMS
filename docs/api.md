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
- `GET /api/devices/:id`
- `GET /api/keypacks`
- `GET /api/system/status`
- `GET /api/audit-logs`
