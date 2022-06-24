use afire::{Content, Method, Response, Server};
use serde_json::json;

use crate::{common::verify_password, App};

pub fn attach(server: &mut Server<App>) {
    server.stateful_route(Method::GET, "/api/admin/home", |app, req| {
        let db = app.db.lock();

        //  Verify password
        if let Some(i) = verify_password(&req, &app.cfg.admin_login) {
            return i;
        }

        let mut out = Vec::new();

        // Get data from db
        let mut querry = db
            .prepare("SELECT name, uuid, latestVersion, creationDate FROM apps")
            .unwrap();
        let mut rows = querry.query([]).unwrap();
        while let Some(i) = rows.next().unwrap() {
            let mut final_latest_version = "0.0.0".to_owned();
            let mut final_version_count = 0;
            let (name, uuid, latest_version, mut final_recent_update) = (
                i.get::<_, String>(0).unwrap(),
                i.get::<_, String>(1).unwrap(),
                i.get::<_, Option<String>>(2).unwrap(),
                i.get::<_, u64>(3).unwrap(),
            );

            let mut querry = db
                .prepare("SELECT versionId, version, creationDate FROM versions WHERE uuid = ?")
                .unwrap();
            let mut rows = querry.query([uuid]).unwrap();
            while let Some(j) = rows.next().unwrap() {
                let (version_id, version, creation_date) = (
                    j.get::<_, String>(0).unwrap(),
                    j.get::<_, String>(1).unwrap(),
                    j.get::<_, u64>(2).unwrap(),
                );

                final_recent_update = final_recent_update.max(creation_date);

                final_version_count += 1;
                if Some(version_id) == latest_version {
                    final_latest_version = version.to_owned();
                }
            }

            out.push(json!({"name": name, "version": final_latest_version, "versions": final_version_count, "recentUpdate": final_recent_update}));
        }
        out.reverse();

        Response::new().content(Content::JSON).text(json!(out))
    })
}
