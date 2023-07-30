DROP TABLE answer;

-- An answer is a user's guess for a card given a session and a deck
CREATE TABLE answer (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER,
    card_id INTEGER NOT NULL,
    deck_id INTEGER NOT NULL,
    session_id INTEGER NOT NULL,
    answer TEXT NOT NULL,
    correct_answer TEXT NOT NULL,
    time DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES user (id),
    FOREIGN KEY (card_id) REFERENCES card (id),
    FOREIGN KEY (deck_id) REFERENCES deck (id),
    FOREIGN KEY (session_id) REFERENCES session (id)
);

