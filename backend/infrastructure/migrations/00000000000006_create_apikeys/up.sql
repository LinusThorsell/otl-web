CREATE TABLE apikeys (
  id SERIAL PRIMARY KEY,
  apikey TEXT NOT NULL,
  expires TIMESTAMP NOT NULL
);

INSERT INTO apikeys (apikey, expires)
VALUES (
    md5(random()::text || clock_timestamp()::text),  -- Generate random API key
    NOW() + interval '90 days'                       -- Set expiration date to 90 days from now
);
