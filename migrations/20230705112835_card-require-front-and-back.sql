DROP TABLE IF EXISTS card;

CREATE TABLE card (
    id INTEGER PRIMARY KEY,
    front TEXT,
    back TEXT,
    created_at DATETIME default CURRENT_TIMESTAMP,
    updated_at DATETIME default CURRENT_TIMESTAMP
);

CREATE TRIGGER update_timestamp
AFTER UPDATE
ON card
FOR EACH ROW
BEGIN
    UPDATE card SET updated_at = CURRENT_TIMESTAMP WHERE id = OLD.id;
END;
