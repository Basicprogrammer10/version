use std::sync::Arc;

use afire::{Content, Method, Request, Response, Server};
use rusqlite::Error;
use serde_json::json;

use crate::{
    common::{text_err_handle, verify_access, ResponseType},
    App,
};

pub fn attach(server: &mut Server<App>) {
    server.stateful_route(Method::GET, "/{app}/version/{version}", |app, req| {
        let res_type = ResponseType::from_headers(&req);

        match process(app, req) {
            Ok(i) => match res_type {
                ResponseType::Text => Response::new().content(Content::TXT).text(format!(
                    "{},{},{}",
                    i.0,
                    i.2,
                    i.1.replace('\r', "").replace('\n', "\\n")
                )),
                ResponseType::Json => Response::new()
                    .content(Content::JSON)
                    .text(json!({"version": i.0, "changelog": i.1, "date": i.2})),
            },
            Err(e) => text_err_handle(e, res_type),
        }
    });
}

fn process<'a>(app: Arc<App>, req: Request) -> Result<(String, String, u64), &'a str> {
    let db = app.db.lock();

    // Get path parms
    let app_name = req.path_param("app").unwrap();
    let version = req.path_param("version").unwrap();

    // Get app UUID
    let (uuid, access_code) = match db.query_row(
        "SELECT uuid, accessCode FROM apps WHERE name = ?",
        [app_name],
        |row| Ok((row.get::<_, String>(0)?, row.get::<_, Option<String>>(1)?)),
    ) {
        Ok(i) => i,
        Err(rusqlite::Error::QueryReturnedNoRows) => return Err("App not found"),
        Err(e) => panic!("{}", e),
    };

    verify_access(&req, access_code)?;

    // Get version and changelog
    let out = match db.query_row(
        "SELECT version, changelog, creationDate FROM versions WHERE version = ? AND uuid = ?",
        [version, uuid],
        |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, u64>(2)?,
            ))
        },
    ) {
        Ok(i) => i,
        Err(Error::QueryReturnedNoRows) => return Err("Version not found"),
        Err(e) => panic!("{}", e),
    };

    // Send JSON response
    Ok(out)
}
