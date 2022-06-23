use std::{io::Read, sync::Arc};

use afire::{Method, Request, Response, Server};
use rusqlite::{DatabaseName, Error};

use crate::{
    common::{text_err_handle, ResponseType},
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
    let (row_id, access_code) = match db.query_row(
        "SELECT ROWID, accessCode FROM versions WHERE version = ? AND uuid = ?",
        [version, uuid],
        |row| Ok((row.get::<_, u64>(0)?, row.get::<_, Option<String>>(1)?)),
    ) {
        Ok(i) => i,
        Err(Error::QueryReturnedNoRows) => return Err("Version not found"),
        Err(e) => panic!("{}", e),
    };

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
    let mut buff = Vec::new();
    let mut blob = match db.blob_open(DatabaseName::Main, "versions", "data", row_id as i64, true)
    {
        Ok(i) => i,
        Err(_) => return Err("No file for version"),
    };
    blob.read_to_end(&mut buff).unwrap();

    Ok(buff)
}
