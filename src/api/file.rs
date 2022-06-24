use std::fs;
use std::sync::Arc;

use afire::{Method, Request, Response, Server};
use rusqlite::{params, Error};

use crate::{
    common::{text_err_handle, verify_access, ResponseType},
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
    let (uuid, access_code) = match db.query_row(
        "SELECT uuid, accessCode FROM apps WHERE name = ?",
        [app_name],
        |row| Ok((row.get::<_, String>(0)?, row.get::<_, Option<String>>(1)?)),
    ) {
        Ok(i) => i,
        Err(rusqlite::Error::QueryReturnedNoRows) => return Err("App not found"),
        Err(e) => panic!("{}", e),
    };

    // Get file
    let file_id = match db.query_row(
        "SELECT file FROM versions WHERE version = ? AND uuid = ?",
        params![version, uuid],
        |row| row.get::<_, Option<String>>(0),
    ) {
        Ok(i) => i,
        Err(Error::QueryReturnedNoRows) => return Err("Version not found"),
        Err(e) => panic!("{}", e),
    };

    if file_id.is_none() {
        return Err("No file for version");
    }

    // Verify access code
    verify_access(&req, access_code)?;

    // Get file
    let buff = fs::read(format!("data/files/{}", file_id.unwrap())).unwrap();
    Ok(buff)
}
