CREATE TABLE IF NOT EXISTS versions (
    uuid TEXT NOT NULL,
    versionId TEXT NOT NULL UNIQUE,
    version TEXT NOT NULL UNIQUE,
    changelog TEXT NOT NULL,
    creationDate INTEGER NOT NULL,
    file TEXT
)