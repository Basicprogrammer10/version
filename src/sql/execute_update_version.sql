UPDATE versions
SET version = ?3,
    changelog = ?4
WHERE uuid = (
        SELECT uuid
        FROM apps
        WHERE name = ?1
    )