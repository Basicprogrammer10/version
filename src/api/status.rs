use std::sync::Arc;

use afire::{Content, Method, Request, Response, Server};
use rusqlite;
use serde_json::json;

use crate::{common::ResponseType, App};

pub fn attach(server: &mut Server<App>) {
    server.stateful_route(Method::GET, "/{app}/status", |app, req| {
        let res_type = ResponseType::fromHeaders(&req);

        match process(app, req) {
            Ok(i) => match res_type {
                ResponseType::Text => Response::new()
                    .content(Content::TXT)
                    .text(format!("{}, {}", i.0, i.1)),
                ResponseType::Json => Response::new()
                    .content(Content::JSON)
                    .text(json!({ "version": i.0, "date": i.1 })),
            },
            Err(e) => match res_type {
                ResponseType::Text => Response::new().status(404).content(Content::TXT).text(e),
                ResponseType::Json => Response::new()
                    .status(404)
                    .content(Content::JSON)
                    .text(json!({"error": "No Versions"})),
            },
        }
    });
}

fn process<'a>(app: Arc<App>, req: Request) -> Result<(String, u64), &'a str> {
    // Get app name
    let app_name = req.path_param("app").unwrap();

    // Get versions
    let latest_version = match app.db.lock().query_row(
        "SELECT latestVersion FROM apps WHERE name = ?",
        [app_name],
        |row| row.get::<_, Option<String>>(0),
    ) {
        Ok(i) => match i {
            Some(i) => i,
            None => return Err("No app versions found"),
        },
        Err(rusqlite::Error::QueryReturnedNoRows) => return Err("App not found"),
        Err(e) => panic!("{}", e),
    };

    // Get version and changelog
    let out = app
        .db
        .lock()
        .query_row(
            "SELECT version, creationDate FROM versions WHERE versionId = ?",
            [latest_version],
            |row| Ok((row.get::<_, String>(0)?, row.get::<_, u64>(1)?)),
        )
        .unwrap();

    // Send JSON response
    Ok(out)
}
