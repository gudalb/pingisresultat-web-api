CREATE TABLE matches (
  id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL,
  body TEXT NOT NULL,
  player_one_name TEXT NOT NULL,
  player_two_name TEXT NOT NULL,
  created_date TIMESTAMP NOT NULL
)