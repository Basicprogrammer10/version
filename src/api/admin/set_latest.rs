use afire::{Content, Method, Response, Server};
use rusqlite::params;
use serde_json::{self, json, Value};
use uuid::Uuid;

use crate::{
    common::{json_err, verify_password},
    App,
};

pub fn attach(server: &mut Server<App>) {
    server.stateful_route(Method::POST, "/api/admin/set_latest", |app, req| {
        // Verify Password
        if let Some(i) = verify_password(&req, &&app.cfg.admin_login) {
            return i;
        }

        // Get data
        let body = serde_json::from_str::<Value>(&req.body_string().unwrap()).unwrap();
        let version = body
            .get("version")
            .unwrap()
            .as_str()
            .map(|x| Uuid::parse_str(x).unwrap().to_string());
        let app_name = body.get("app").unwrap().as_str().unwrap();

        // Update Database
        if version.is_some()
            && app
                .db
                .lock()
                .query_row(
                    "SELECT Count(*) FROM versions WHERE versionId = ?",
                    [&version],
                    |row| row.get::<_, u64>(0),
                )
                .unwrap()
                < 1
        {
            return json_err("Invalid version");
        }

        app.db
            .lock()
            .execute(
                "UPDATE apps SET latestVersion = ? WHERE name = ?",
                params![version, app_name],
            )
            .unwrap();

        // Send response
        Response::new()
            .content(Content::JSON)
            .text(json!({"status": "ok"}))
    });
}
