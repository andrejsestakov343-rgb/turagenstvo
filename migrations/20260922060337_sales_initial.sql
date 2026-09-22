CREATE TABLE IF NOT EXISTS sales (
    id              BLOB PRIMARY KEY NOT NULL,
    route_id        BLOB NOT NULL,
    travel          TEXT NOT NULL,
    price           REAL NOT NULL,
    quantity        INTEGER NOT NULL,
    sale_date       TEXT NOT NULL,
    created_at      TEXT NOT NULL,
    finished_at     TEXT,
    FOREIGN KEY (route_id) REFERENCES routes(id)
);
