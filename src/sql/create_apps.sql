CREATE TABLE IF NOT EXISTS apps (
    uuid TEXT NOT NULL,
    name TEXT NOT NULL,
    latestVersion TEXT,
    creationDate INTEGER NOT NULL,
    accessCode TEXT,
    UNIQUE(uuid)
)