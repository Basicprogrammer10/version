use afire::{Content, Method, Response, Server};
use rusqlite::params;
use serde_json::{self, json, Value};
use uuid::Uuid;

use crate::{common::verify_password, App};

pub fn attach(server: &mut Server<App>) {
    server.stateful_route(Method::POST, "/api/admin/new_version", |app, req| {
        // Verify Password
        if let Some(i) = verify_password(&req, &&app.cfg.admin_login) {
            return i;
        }

        // Get data
        let body = serde_json::from_str::<Value>(&req.body_string().unwrap()).unwrap();
        let app_name = body.get("app").unwrap().as_str().unwrap();
        let version = body.get("version").unwrap().as_str().unwrap();
        let changelog = body.get("changelog").unwrap().as_str().unwrap();
        let editing = body
            .get("edit")
            .unwrap_or(&Value::Bool(false))
            .as_bool()
            .unwrap();

        let version_id = Uuid::new_v4().to_string();

        // Update Database
        app.db
            .lock()
            .execute(
                if editing {
                    include_str!("../../sql/execute_update_version.sql")
                } else {
                    include_str!("../../sql/execute_new_version.sql")
                },
                params![app_name, version_id, version, changelog],
            )
            .unwrap();

        // Send response
        Response::new()
            .content(Content::JSON)
            .text(json!({"status": "ok"}))
    });
}
