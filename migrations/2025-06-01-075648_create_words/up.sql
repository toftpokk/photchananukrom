CREATE TABLE words (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    word TEXT NOT NULL
);

CREATE TABLE definitions (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    word_id INTEGER NOT NULL,
    definition TEXT NOT NULL,
    definition_header TEXT NOT NULL,
    FOREIGN KEY (word_id) 
        REFERENCES words(id)
        ON DELETE CASCADE
);