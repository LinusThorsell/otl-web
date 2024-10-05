CREATE TABLE tours (
  id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL,
  location VARCHAR NOT NULL,
  description VARCHAR NOT NULL,
  body VARCHAR NOT NULL,
  start_date TIMESTAMP NOT NULL,
  end_date TIMESTAMP NOT NULL,
  url VARCHAR NOT NULL,
  score_count INTEGER NOT NULL,
  image VARCHAR NOT NULL
)
