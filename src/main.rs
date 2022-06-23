use std::fs;
use std::path::Path;

use afire::{
    extension::{Logger, ServeStatic},
    Middleware, Server,
};

mod api;
mod app;
mod common;
mod r#static;
use app::App;

fn main() {
    // Make filr dir
    assert!(Path::new("data/files").exists() || fs::create_dir("data/files").is_ok());

    let app = App::new();
    let mut server = Server::<App>::new(&app.cfg.host, app.cfg.port).state(app);
    ServeStatic::new("web/static").attach(&mut server);
    Logger::new().attach(&mut server);

    api::attach(&mut server);
    r#static::attach(&mut server);

    server.start().unwrap();
}
