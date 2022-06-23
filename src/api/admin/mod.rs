use afire::Server;

use crate::App;

mod edit;
mod home;
mod set_file;

pub fn attach(server: &mut Server<App>) {
    edit::attach(server);
    home::attach(server);
    set_file::attach(server);
}
