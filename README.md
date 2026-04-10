# Mini KMS Admin

Mini KMS Admin is a learning project for practicing:

- frontend page organization
- API integration
- backend layering
- auth flow
- CRUD pages
- audit logs

## Folders

- `frontend/`: SolidJS admin app
- `backend/`: Rust + Axum API service
- `database/schema.sql`: MySQL schema design snapshot
- `database/migrations/`: versioned SQL migrations
- `docs/api.md`: API list

## Features

- login with account and password
- bearer token auth
- dashboard status page
- user CRUD
- device list and detail
- keypack list
- audit log list
- audit records for login, create user, update user, delete user

## Structure

```text
mini-kms-admin
|- frontend
|  |- src/routes
|  |- src/lib/api
|  |- src/lib/models
|  |- src/lib/storage
|  |- src/components
|- backend
|  |- src/routes
|  |- src/service
|  |- src/middleware
|  |- src/state
|  |- src/models
|  |- src/entity
|  |- src/error
|- docs
|- database
```

## Stack

Frontend:

- SolidJS
- Vite
- Tailwind CSS
- ky
- @tanstack/solid-query
- @solidjs/router

Backend:

- Rust
- Axum
- Tokio
- Serde
- JsonWebToken
- Argon2

The backend now reads data from MySQL through SeaORM repository calls.
See `database/README.md` for schema apply, seed data, and verification commands.

## Auth And Data Flow

1. User signs in on `/login`.
2. Frontend calls `POST /api/login`.
3. Token is saved in `localStorage`.
4. The shared `ky` client adds `Authorization: Bearer <token>` automatically.
5. Backend auth middleware validates the token.
6. Services handle business logic and append audit logs for key actions.

## Default Account

- account: `admin`
- password: `admin123`
- account: `operator`
- password: `operator123`

## Run

### 1) Prepare Database (MySQL)

```sql
CREATE DATABASE IF NOT EXISTS mini_kms_admin
  DEFAULT CHARACTER SET utf8mb4
  DEFAULT COLLATE utf8mb4_unicode_ci;
USE mini_kms_admin;
SOURCE database/migrations/001_init.sql;
SOURCE database/seed.sql;
```

### 2) Run Backend

```bash
cd backend
# PowerShell
$env:DATABASE_URL="mysql://<user>:<password>@127.0.0.1:3306/mini_kms_admin"

# bash
export DATABASE_URL="mysql://<user>:<password>@127.0.0.1:3306/mini_kms_admin"

cargo run --bin mini-kms-admin-backend
```

Runs on `http://127.0.0.1:3000`

### 3) Run Frontend

```bash
cd frontend
npm install
npm run dev
```

Runs on `http://127.0.0.1:11037`
