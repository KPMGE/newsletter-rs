CREATE TABLE subscriptions (
  id            UUID NOT NULL,
  email         TEXT NOT NULL UNIQUE, 
  name          TEXT NOT NULL,
  subscribed_at TIMESTAMPTZ NOT NULL,
  PRIMARY KEY(id)
);
