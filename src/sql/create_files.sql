CREATE TABLE IF NOT EXISTS files (
    uuid TEXT NOT NULL,
    blob BLOB NOT NULL,
    creationDate INTEGER NOT NULL,
    accessCode TEXT NOT NULL,
    UNIQUE(uuid)
)