-- A tag is a label that can be applied to a card
CREATE TABLE tag (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    description TEXT,
    UNIQUE (name)
);

-- A card can have many tags
CREATE TABLE card_tag (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    card_id INTEGER NOT NULL,
    tag_id INTEGER NOT NULL,
    FOREIGN KEY (card_id) REFERENCES card (id),
    FOREIGN KEY (tag_id) REFERENCES tag (id),
    UNIQUE (card_id, tag_id)
);
