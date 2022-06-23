CREATE TABLE IF NOT EXISTS versions (
    uuid TEXT NOT NULL,
    versionId TEXT NOT NULL,
    version TEXT NOT NULL,
    changelog TEXT NOT NULL,
    creationDate INTEGER NOT NULL,
    
    -- File
    data BLOB,
    accessCode TEXT
)