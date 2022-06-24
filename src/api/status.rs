use std::sync::Arc;

use afire::{Content, Method, Request, Response, Server};
use serde_json::json;

use crate::{
    common::{text_err_handle, verify_access, ResponseType},
    App,
};

pub fn attach(server: &mut Server<App>) {
    server.stateful_route(Method::GET, "/{app}/status", |app, req| {
        let res_type = ResponseType::from_headers(&req);

        match process(app, req) {
            Ok(i) => match res_type {
                ResponseType::Text => Response::new()
                    .content(Content::TXT)
                    .text(format!("{},{}", i.0, i.1)),
                ResponseType::Json => Response::new()
                    .content(Content::JSON)
                    .text(json!({ "version": i.0, "date": i.1 })),
            },
            Err(e) => text_err_handle(e, res_type),
        }
    });
}

fn process<'a>(app: Arc<App>, req: Request) -> Result<(String, u64), &'a str> {
    // Get app name
    let app_name = req.path_param("app").unwrap();

    // Get versions
    let (latest_version, access_code) = match app.db.lock().query_row(
        "SELECT latestVersion, accessCode FROM apps WHERE name = ?",
        [app_name],
        |row| {
            Ok((
                row.get::<_, Option<String>>(0)?,
                row.get::<_, Option<String>>(1)?,
            ))
        },
    ) {
        Ok(i) => match i {
            (Some(i), j) => (i, j),
            (None, _) => return Err("No app versions found"),
        },
        Err(rusqlite::Error::QueryReturnedNoRows) => return Err("App not found"),
        Err(e) => panic!("{}", e),
    };

    verify_access(&req, access_code)?;

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
