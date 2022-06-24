use afire::Server;

use crate::App;

mod edit;
mod home;
mod new_version;
mod set_file;
mod set_latest;

pub fn attach(server: &mut Server<App>) {
    edit::attach(server);
    home::attach(server);
    new_version::attach(server);
    set_file::attach(server);
    set_latest::attach(server);
}
