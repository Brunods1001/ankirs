-- Create a sqlite Deck table
CREATE TABLE deck (
  id INTEGER PRIMARY KEY,
  name TEXT NOT NULL,
  description TEXT,
  created_at DATETIME NOT NULL,
  updated_at DATETIME NOT NULL
);

-- Create a CardDeck table with (card_id, deck_id) as a composite primary key
CREATE TABLE card_deck (
    card_id INTEGER NOT NULL,
    deck_id INTEGER NOT NULL,
    PRIMARY KEY (card_id, deck_id),
    FOREIGN KEY (card_id) REFERENCES card(id),
    FOREIGN KEY (deck_id) REFERENCES deck(id)
);

