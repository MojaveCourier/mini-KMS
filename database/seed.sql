-- Seed data for local development.
-- Password for admin account is: admin123

INSERT INTO users (account, password_hash, role, created_at)
VALUES
  (
    'admin',
    '$argon2id$v=19$m=19456,t=2,p=1$HkA28BX2T9afZ6PITv9ewA$k042/JlQ1h/uNgVI1Y5d/MqIcdJDDQpjWtDQAzGrNYg',
    'admin',
    NOW()
  ),
  (
    'operator',
    '$argon2id$v=19$m=19456,t=2,p=1$cOGGbeOZRUGHIEg1Zu4yiQ$PEFeUEeY56gyE/uFOFgQj6fz0L+POYwraaURJDkNp7w',
    'operator',
    NOW()
  );

INSERT INTO devices (serial, name, status, last_seen_at, created_at)
VALUES
  ('DEV-0001', 'Shanghai Signing Box', 'active', NOW() - INTERVAL 10 MINUTE, NOW() - INTERVAL 30 DAY),
  ('DEV-0002', 'Hangzhou Backup Box', 'inactive', NOW() - INTERVAL 3 DAY, NOW() - INTERVAL 20 DAY),
  ('DEV-0003', 'Beijing Test Box', 'active', NOW() - INTERVAL 45 MINUTE, NOW() - INTERVAL 5 DAY);

INSERT INTO keypacks (device_id, version, status, created_at)
VALUES
  (1, 'v1.0.0', 'issued', NOW() - INTERVAL 12 DAY),
  (1, 'v1.1.0', 'active', NOW() - INTERVAL 2 DAY),
  (3, 'v0.9.1', 'draft', NOW() - INTERVAL 36 HOUR);

INSERT INTO audit_logs (user_id, action, target_type, target_id, detail, created_at)
VALUES
  (1, 'SEED', 'system', NULL, 'seeded demo data', NOW() - INTERVAL 15 DAY);
