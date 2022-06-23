use afire::{Content, Method, Response, Server};
use serde_json::json;
use sha2::{Digest, Sha256};

use crate::{common::json_err, App};

pub fn attach(server: &mut Server<App>) {
    server.stateful_route(Method::GET, "/api/admin/home", |app, req| {
        let db = app.db.lock();

        // Get password
        let password = match req.header("password") {
            Some(i) => i,
            None => return json_err("No password"),
        };

        // Check password
        let mut hasher = Sha256::new();
        hasher.update(password.as_bytes().to_vec());
        let hash = format!("{:x}", hasher.finalize());

        if hash != app.cfg.admin_login {
            return json_err("Incorrect password");
        }

        let mut out = Vec::new();

        // Get data from db
        let mut querry = db
            .prepare("SELECT name, uuid, latestVersion FROM apps")
            .unwrap();
        let mut rows = querry.query([]).unwrap();
        while let Some(i) = rows.next().unwrap() {
            let mut final_latest_version = "0.0.0".to_owned();
            let mut final_version_count = 0;
            let mut final_recent_update = 0;
            let (name, uuid, latest_version) = (
                i.get::<_, String>(0).unwrap(),
                i.get::<_, String>(1).unwrap(),
                i.get::<_, String>(2).unwrap(),
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
                if version_id == latest_version {
                    final_latest_version = version.to_owned();
                }
            }

            out.push(json!({"name": name, "version": final_latest_version, "versions": final_version_count, "recentUpdate": final_recent_update}));
        }

        Response::new().content(Content::JSON).text(json!(out))
    })
}
