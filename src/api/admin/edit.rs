use afire::{Content, Method, Response, Server};
use serde_json::json;

use crate::{
    common::{json_err, verify_password},
    App,
};

pub fn attach(server: &mut Server<App>) {
    server.stateful_route(Method::GET, "/api/admin/edit/{app}", |app, req| {
        let db = app.db.lock();

        // Verify password
        if let Some(i) = verify_password(&req, &app.cfg.admin_login) {
            return i;
        }

        // Get app
        let app_name = req.path_param("app").unwrap();
        let (uuid, latest_version) = match db.query_row(
            "SELECT uuid, latestVersion FROM apps WHERE name = ?",
            [&app_name],
            |row| Ok((row.get::<_, String>(0)?, row.get::<_, Option<String>>(1)?)),
        ) {
            Ok(i) => i,
            Err(rusqlite::Error::QueryReturnedNoRows) => return json_err("App not found"),
            Err(e) => panic!("{}", e),
        };

        // Get app versions
        let mut versions = Vec::new();
        let mut final_latest_version = None;

        let mut querry = db
            .prepare("SELECT file, version, versionId, changelog, creationDate, accessCode FROM versions WHERE uuid = ?")
            .unwrap();
        let mut rows = querry.query([uuid]).unwrap();
        while let Some(i) = rows.next().unwrap() {
            let (file, version, version_id, changelog, creation_data, access_code) = (
                    i.get::<_, Option<String>>(0).unwrap().is_some(),
                    i.get::<_, String>(1).unwrap(),
                    i.get::<_, String>(2).unwrap(),
                    i.get::<_, String>(3).unwrap(),
                    i.get::<_, u64>(4).unwrap(),
                    i.get::<_, Option<String>>(5).unwrap(),
                );

            if Some(version_id.to_owned()) == latest_version {
                final_latest_version = Some(version.clone());
            }

            versions.push(json!({"version": version, "id": version_id, "changelog": changelog, "date": creation_data, "access": access_code, "file": file}));
        }
        versions.reverse();

        // Send JSON
        Response::new().content(Content::JSON).text(json!({"name": app_name, "version": final_latest_version, "versions": versions}))
    })
}
