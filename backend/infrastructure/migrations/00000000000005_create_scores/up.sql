CREATE TABLE scores (
  id SERIAL PRIMARY KEY,
  score INTEGER NOT NULL,
  divcode TEXT NOT NULL,

  event_id INTEGER NOT NULL REFERENCES events(id),
  user_id INTEGER NOT NULL REFERENCES users(id)
);
