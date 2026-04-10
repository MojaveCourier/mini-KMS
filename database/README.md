# Database Setup

Current scope:

- clear domain constraints (roles/status values)
- foreign key policies
- baseline indexes for list/filter/sort scenarios
- versioned SQL migration script
- seed data for local development

## Files

- `schema.sql`: latest target schema snapshot
- `migrations/001_init.sql`: initial migration SQL
- `seed.sql`: demo users/devices/keypacks/audit logs

## Apply In MySQL

```sql
CREATE DATABASE IF NOT EXISTS mini_kms_admin
  DEFAULT CHARACTER SET utf8mb4
  DEFAULT COLLATE utf8mb4_unicode_ci;
USE mini_kms_admin;
SOURCE database/migrations/001_init.sql;
SOURCE database/seed.sql;
```

If your SQL client does not support `SOURCE`, run the file content manually after `USE mini_kms_admin`.

## Verify

```sql
SHOW TABLES;
SHOW CREATE TABLE users;
SHOW CREATE TABLE devices;
SHOW CREATE TABLE keypacks;
SHOW CREATE TABLE audit_logs;
SELECT account, role FROM users;
```

## Password Hash Helper

If you need to create your own seed users with new passwords:

```bash
cd backend
cargo run --bin hash_password -- your_password_here
```

Use the printed hash in `INSERT INTO users (... password_hash ...)`.
