use std::fs;
use std::{io::Read, sync::Arc};

use afire::{Method, Request, Response, Server};
use rusqlite::{DatabaseName, Error};

use crate::{
    common::{json_err, text_err_handle, ResponseType},
    App,
};

pub fn attach(server: &mut Server<App>) {
    server.stateful_route(Method::GET, "/{app}/file/{version}", |app, req| {
        let res_type = ResponseType::from_headers(&req);

        match process(app, req) {
            Ok(i) => Response::new().bytes(i),
            Err(e) => text_err_handle(e, res_type),
        }
    });
}

fn process<'a>(app: Arc<App>, req: Request) -> Result<Vec<u8>, &'a str> {
    let db = app.db.lock();

    // Get path parms
    let app_name = req.path_param("app").unwrap();
    let version = req.path_param("version").unwrap();

    // Get app UUID
    let uuid = match db.query_row("SELECT uuid FROM apps WHERE name = ?", [app_name], |row| {
        row.get::<_, String>(0)
    }) {
        Ok(i) => i,
        Err(rusqlite::Error::QueryReturnedNoRows) => return Err("App not found"),
        Err(e) => panic!("{}", e),
    };

    // Get file
    let (file_id, access_code) = match db.query_row(
        "SELECT file, accessCode FROM versions WHERE version = ? AND uuid = ?",
        [version, uuid],
        |row| {
            Ok((
                row.get::<_, Option<String>>(0)?,
                row.get::<_, Option<String>>(1)?,
            ))
        },
    ) {
        Ok(i) => i,
        Err(Error::QueryReturnedNoRows) => return Err("Version not found"),
        Err(e) => panic!("{}", e),
    };

    if file_id.is_none() {
        return Err("No file for version");
    }

    // Verify access code
    if let Some(access_code) = access_code {
        if !access_code.is_empty() {
            let access_attempt = match req.query.get("code") {
                Some(i) => i,
                None => return Err("No Access Code"),
            };

            if access_attempt != access_code {
                return Err("Invalid Access Code");
            }
        }
    }

    // Get file
    let buff = fs::read(format!("data/files/{}", file_id.unwrap())).unwrap();
    Ok(buff)
}
