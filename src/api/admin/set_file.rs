use std::{io::Write, str::FromStr};

use afire::{internal::common::decode_url, Content, Method, Response, Server};
use rusqlite::{params, DatabaseName, Error};
use serde_json::json;
use uuid::Uuid;

use crate::{
    common::{json_err, verify_password},
    App,
};

pub fn attach(server: &mut Server<App>) {
    server.stateful_route(Method::POST, "/api/admin/set_file", |app, req| {
        if req.body.len() > 1_000_000_000 {
            return json_err("File too big");
        }

        // Verify Password
        if let Some(i) = verify_password(&req, &app.cfg.admin_login) {
            return i;
        }

        // Get headers
        let version_id = match req.header("id") {
            Some(i) => match Uuid::from_str(&i) {
                Ok(i) => i,
                Err(_) => return json_err("Invalid UUID"),
            },
            None => return json_err("No version ID"),
        }
        .to_string();

        let access = req.header("access").map(decode_url);

        // Update access code
        let db = app.db.lock();

        let row_id = match db.query_row(
            "SELECT ROWID FROM versions WHERE versionId = ?",
            [&version_id],
            |row| row.get::<_, u64>(0),
        ) {
            Ok(i) => i,
            Err(Error::QueryReturnedNoRows) => return json_err("Invalid version"),
            Err(e) => panic!("{}", e),
        };

        if let Some(i) = access {
            db.execute(
                "UPDATE versions SET accessCode = ? WHERE versionId = ?",
                params![i, version_id],
            )
            .unwrap();
        }

        // Update data
        if !req.body.is_empty() {
            let mut data =
                match db.blob_open(DatabaseName::Main, "versions", "data", row_id as i64, false) {
                    Ok(i) => i,
                    Err(e) => panic!("{}", e),
                };
            data.write_all(&req.body).unwrap();
        }

        Response::new()
            .content(Content::JSON)
            .text(json!({"status": "ok"}))
    });
}
