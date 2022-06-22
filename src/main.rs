use afire::{extension::Logger, middleware::Middleware, Server};

mod app;
mod api;
mod common;
use app::App;

fn main() {
    let app = App::new();

    let mut server = Server::<App>::new(&app.cfg.host, app.cfg.port).state(app);
    Logger::new().attach(&mut server);

    api::attach(&mut server);

    server.start().unwrap();
}
