use afire::Server;

use crate::App;

mod status;

pub fn attach(server: &mut Server<App>) {
    status::attach(server);
}