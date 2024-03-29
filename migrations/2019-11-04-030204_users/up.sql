CREATE TABLE users (
  user_id SERIAL PRIMARY KEY,
  user_uuid UUID NOT NULL,
  hash BYTEA NOT NULL,
  salt VARCHAR(255) NOT NULL,
  email VARCHAR(120) NOT NULL UNIQUE,
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  name VARCHAR NOT NULL
);

CREATE UNIQUE INDEX email_idx ON users(email);