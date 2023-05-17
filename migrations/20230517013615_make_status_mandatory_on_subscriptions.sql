BEGIN;
  -- update status of subscriptions to confirmed
  UPDATE subscriptions
  SET status = 'confirmed'
  WHERE status IS NULL;
  -- make status column mandatory
  ALTER TABLE subscriptions ALTER COLUMN status SET NOT NULL;
COMMIT;
