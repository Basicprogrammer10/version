CREATE TABLE IF NOT EXISTS files (
    uuid TEXT NOT NULL,
    data BLOB NOT NULL,
    creationDate INTEGER NOT NULL,
    accessCode TEXT,
    UNIQUE(uuid)
)