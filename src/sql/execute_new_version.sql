INSERT INTO versions (
        uuid,
        versionId,
        version,
        changelog,
        creationDate
    )
VALUES (
        (
            SELECT uuid
            FROM apps
            WHERE name = ?
        ),
        ?,
        ?,
        ?,
        strftime('%s', 'now')
    )