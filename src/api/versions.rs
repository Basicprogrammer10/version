use std::str::FromStr;
use std::sync::Arc;

use afire::{Content, Method, Request, Response, Server};
use semver::{BuildMetadata, Prerelease, Version};
use serde_json::json;

use crate::{
    common::{text_err_handle, verify_access, ResponseType},
    App,
};

pub fn attach(server: &mut Server<App>) {
    server.stateful_route(Method::GET, "/{app}/versions", |app, req| {
        let res_type = ResponseType::from_headers(&req);

        match process(app, req) {
            Ok(i) => match res_type {
                ResponseType::Text => Response::new().content(Content::TXT).text(
                    i.iter()
                        .map(|x| {
                            format!(
                                "{},{},{}",
                                x.0,
                                x.2,
                                x.1.replace('\r', "").replace('\n', "\\n")
                            )
                        })
                        .collect::<Vec<String>>()
                        .join("\n"),
                ),
                ResponseType::Json => Response::new().content(Content::JSON).text(json!(i
                    .iter()
                    .map(|x| { json!({"version": x.0, "changelog": x.1, "date": x.2}) })
                    .collect::<Vec<_>>())),
            },
            Err(e) => text_err_handle(e, res_type),
        }
    });
}

fn process<'a>(app: Arc<App>, req: Request) -> Result<Vec<(String, String, u64)>, &'a str> {
    let db = app.db.lock();

    // Get app name
    let app_name = req.path_param("app").unwrap();

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

    // Get all versions from app
    let mut query = db
        .prepare("SELECT version, changelog, creationDate FROM versions WHERE uuid = ?")
        .unwrap();
    let mut rows = query.query([uuid]).unwrap();
    let mut out = Vec::new();

    while let Some(i) = rows.next().unwrap() {
        let ver = i.get::<_, String>(0).unwrap();

        out.push((
            Version::from_str(&ver).unwrap_or(Version {
                major: 0,
                minor: 0,
                patch: 0,
                pre: Prerelease::EMPTY,
                build: BuildMetadata::EMPTY,
            }),
            (
                ver,
                i.get::<_, String>(1).unwrap(),
                i.get::<_, u64>(2).unwrap(),
            ),
        ));
    }

    // Sort versions
    out.sort_by(|a, b| b.0.cmp(&a.0));

    Ok(out.iter().map(|x| x.1.to_owned()).collect())
}
