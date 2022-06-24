use std::fs;
use std::str::FromStr;

use afire::{Content, Method, Response, Server};
use rusqlite::params;
use serde_json::json;
use uuid::Uuid;

use crate::{
    common::{json_err, verify_password},
    App,
};

pub fn attach(server: &mut Server<App>) {
    server.stateful_route(Method::POST, "/api/admin/set_file", |app, req| {
        // Verify Password
        if let Some(i) = verify_password(&req, &app.cfg.admin_login) {
            return i;
        }

        // Get headers
        let new_file = !req.body.is_empty();
        let version_id = match req.header("id") {
            Some(i) => match Uuid::from_str(&i) {
                Ok(i) => i,
                Err(_) => return json_err("Invalid UUID"),
            },
            None => return json_err("No version ID"),
        }
        .to_string();

        // Update data
        let file_path = app
            .db
            .lock()
            .query_row(
                "SELECT file FROM versions WHERE versionId = ?",
                [&version_id],
                |row| row.get::<_, Option<String>>(0),
            )
            .unwrap();

        match process_file(file_path, new_file, req.body) {
            FileAction::Nothing => {}
            FileAction::Insert(uuid) => {
                app.db
                    .lock()
                    .execute(
                        "UPDATE versions SET file = ? WHERE versionId = ?",
                        params![uuid.to_string(), &version_id],
                    )
                    .unwrap();
            }
            FileAction::Delete => {
                app.db
                    .lock()
                    .execute(
                        "UPDATE versions SET file = NULL WHERE versionId = ?",
                        [&version_id],
                    )
                    .unwrap();
            }
        }

        Response::new()
            .content(Content::JSON)
            .text(json!({"status": "ok"}))
    });
}

fn process_file(file_path: Option<String>, new_file: bool, body: Vec<u8>) -> FileAction {
    if let Some(i) = file_path {
        if new_file {
            fs::write(format_path(i), body).unwrap();
            return FileAction::Nothing;
        }
        fs::remove_file(format_path(i)).unwrap();
        return FileAction::Delete;
    }
    if !new_file {
        return FileAction::Nothing;
    }

    let new_path = Uuid::new_v4();
    fs::write(format_path(new_path.to_string()), body).unwrap();
    FileAction::Insert(new_path)
}

fn format_path(uuid: String) -> String {
    format!("data/files/{}", uuid)
}

enum FileAction {
    Nothing,
    Delete,
    Insert(Uuid),
}
