use afire::Server;

use crate::App;

mod home;

pub fn attach(server: &mut Server<App>) {
    home::attach(server);
}
