CREATE TABLE IF NOT EXISTS apps (
    uuid TEXT NOT NULL,
    name TEXT NOT NULL,
    latestVersion,
    creationDate INTEGER NOT NULL,
    UNIQUE(uuid)
)