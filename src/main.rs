use afire::{
    extension::{Logger, ServeStatic},
    Middleware, Server,
};

mod api;
mod app;
mod common;
use app::App;

fn main() {
    let app = App::new();

    let mut server = Server::<App>::new(&app.cfg.host, app.cfg.port).state(app);
    ServeStatic::new("web").attach(&mut server);
    Logger::new().attach(&mut server);

    api::attach(&mut server);

    server.start().unwrap();
}
