use std::fs;
use std::path::Path;

use afire::{
    extension::{Logger, ServeStatic},
    Error, Middleware, Response, Server,
};
use serde_json::json;

mod api;
mod app;
mod common;
mod r#static;
use app::App;

use crate::common::ResponseType;

fn main() {
    // Make filr dir
    assert!(Path::new("data/files").exists() || fs::create_dir("data/files").is_ok());

    let app = App::new();
    let mut server = Server::<App>::new(&app.cfg.host, app.cfg.port).state(app);
    ServeStatic::new("web/static").attach(&mut server);
    Logger::new().attach(&mut server);

    api::attach(&mut server);
    r#static::attach(&mut server);

    server.error_handler(|req, err| match req {
        Ok(i) => {
            return match ResponseType::from_headers(&i) {
                ResponseType::Text => Response::new().status(500).text(err),
                ResponseType::Json => Response::new().status(500).text(json!({ "error": err })),
            }
        }
        Err(_) => Response::new(),
    });

    server.start().unwrap();
}
