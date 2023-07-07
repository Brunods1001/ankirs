DROP TABLE IF EXISTS deck;

-- Create a sqlite Deck table with a unique name
CREATE TABLE deck (
  id INTEGER PRIMARY KEY,
  name TEXT NOT NULL UNIQUE,
  description TEXT,
  created_at DATETIME NOT NULL default CURRENT_TIMESTAMP,
  updated_at DATETIME NOT NULL default CURRENT_TIMESTAMP
);
