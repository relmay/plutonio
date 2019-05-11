CREATE TABLE IF NOT EXISTS transactions (
  id            INTEGER   PRIMARY KEY,
  account_id    INTEGER   NOT NULL,
  title         TEXT,
  t_type        INTEGER   NOT NULL,
  amount        INTEGER   NOT NULL,
  FOREIGN KEY(account_id) REFERENCES accounts(id)
);