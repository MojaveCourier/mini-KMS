CREATE TABLE users (
  id BIGINT PRIMARY KEY AUTO_INCREMENT,
  account VARCHAR(64) NOT NULL,
  password_hash VARCHAR(255) NOT NULL,
  role ENUM('admin', 'operator', 'viewer') NOT NULL,
  created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  UNIQUE KEY uk_users_account (account)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE devices (
  id BIGINT PRIMARY KEY AUTO_INCREMENT,
  serial VARCHAR(64) NOT NULL,
  name VARCHAR(128) NOT NULL,
  status ENUM('active', 'inactive') NOT NULL,
  last_seen_at DATETIME NULL,
  created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  UNIQUE KEY uk_devices_serial (serial),
  KEY idx_devices_status (status),
  KEY idx_devices_last_seen_at (last_seen_at)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE keypacks (
  id BIGINT PRIMARY KEY AUTO_INCREMENT,
  device_id BIGINT NOT NULL,
  version VARCHAR(32) NOT NULL,
  status ENUM('draft', 'issued', 'active', 'revoked') NOT NULL,
  created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  KEY idx_keypacks_device_id (device_id),
  KEY idx_keypacks_created_at (created_at),
  CONSTRAINT fk_keypacks_device_id
    FOREIGN KEY (device_id)
    REFERENCES devices(id)
    ON UPDATE CASCADE
    ON DELETE RESTRICT
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE audit_logs (
  id BIGINT PRIMARY KEY AUTO_INCREMENT,
  user_id BIGINT NULL,
  action VARCHAR(64) NOT NULL,
  target_type VARCHAR(64) NOT NULL,
  target_id BIGINT NULL,
  detail VARCHAR(255) NULL,
  created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  KEY idx_audit_logs_created_at (created_at),
  KEY idx_audit_logs_user_created_at (user_id, created_at),
  KEY idx_audit_logs_target (target_type, target_id),
  CONSTRAINT fk_audit_logs_user_id
    FOREIGN KEY (user_id)
    REFERENCES users(id)
    ON UPDATE CASCADE
    ON DELETE SET NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
