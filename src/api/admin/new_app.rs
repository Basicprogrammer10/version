use afire::{Content, Method, Response, Server};
use rusqlite::params;
use serde_json::{self, json, Value};
use uuid::Uuid;

use crate::{common::verify_password, App};

pub fn attach(server: &mut Server<App>) {
    server.stateful_route(Method::POST, "/api/admin/new_app", |app, req| {
        // Verify Password
        if let Some(i) = verify_password(&req, &&app.cfg.admin_login) {
            return i;
        }

        // Get data
        let body = serde_json::from_str::<Value>(&req.body_string().unwrap()).unwrap();
        let app_name = body.get("name").unwrap().as_str().unwrap();

        let uuid = Uuid::new_v4();

        // Update Database
        app.db
            .lock()
            .execute(
                "INSERT INTO apps (uuid, name, creationDate) VALUES (?, ?, strftime('%s', 'now'))",
                params![uuid.to_string(), app_name],
            )
            .unwrap();
        // Send response
        Response::new()
            .content(Content::JSON)
            .text(json!({"status": "ok"}))
    });
}
