CREATE TABLE IF NOT EXISTS countries (
    id                  BLOB PRIMARY KEY NOT NULL,
    name                TEXT NOT NULL,
    visa                REAL NOT NULL,
    created_at          TEXT NOT NULL,
    updated_at          TEXT,
    deleted_at          TEXT
);
