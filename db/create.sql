CREATE EXTENSION IF NOT EXISTS pgcrypto;

CREATE TABLE IF NOT EXISTS users (
  id uuid PRIMARY KEY,
  name VARCHAR(255) NOT NULL,
  email VARCHAR(255) NOT NULL,
  password VARCHAR(255) NOT NULL,
  salt VARCHAR(255) NOT NULL
);

-- salt/password
INSERT INTO users (id, name, email, password, salt)
VALUES (gen_random_uuid(), 'John Doe', 'john@example.com', '13601bda4ea78e55a07b98866d2be6be0744e3866f13c00c811cab608a28f322', 'c2FsdA==');
