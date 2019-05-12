CREATE TABLE IF NOT EXISTS accounts (
  id        INTEGER PRIMARY KEY,
  title     TEXT    NOT NULL UNIQUE,
  currency  TEXT    NOT NULL,
  balance   INTEGER NOT NULL DEFAULT 0
);