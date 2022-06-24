use std::str::FromStr;

use afire::{internal::common::decode_url, Content, Method, Response, Server};
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
        let editing = body.get("editing").unwrap_or(&Value::Null).as_bool().unwrap_or(false);
        let uuid = if editing {
            Uuid::from_str(&body.get("uuid").unwrap().as_str().unwrap()).unwrap().to_string()
        } else {
            Uuid::new_v4().to_string()
        };
        let access = body
            .get("access")
            .unwrap_or(&Value::Null)
            .as_str()
            .map(|x| decode_url(x.to_string()));

        // Update Database
        app.db
            .lock()
            .execute(
                if editing {
                    "UPDATE apps SET name = ?2, accessCode = ?3 WHERE uuid = ?1"
                } else {
                    "INSERT INTO apps (uuid, name, accessCode, creationDate) VALUES (?, ?, ?, strftime('%s', 'now'))"
                },
                params![uuid, app_name, access],
            )
            .unwrap();
        // Send response
        Response::new()
            .content(Content::JSON)
            .text(json!({"status": "ok"}))
    });
}
