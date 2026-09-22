CREATE TABLE IF NOT EXISTS routes (
    id              BLOB PRIMARY KEY NOT NULL,
    country_id      BLOB NOT NULL,
    name            TEXT NOT NULL,
    created_at      TEXT NOT NULL,
    updated_at      TEXT,
    deleted_at      TEXT,
    FOREIGN KEY (country_id) REFERENCES countries(id)
);
