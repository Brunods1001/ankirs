-- Create a self assessment table with a foreign key to the answer table
-- should use a number from 0 to 100 to represent the self assessment
CREATE TABLE self_assessment (
    id SERIAL PRIMARY KEY,
    answer_id INTEGER REFERENCES answer(id),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    score INTEGER NOT NULL CHECK(score BETWEEN 0 AND 100)
);
