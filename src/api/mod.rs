use afire::Server;

use crate::App;

mod admin;
mod file;
mod status;
mod version;
mod versions;

pub fn attach(server: &mut Server<App>) {
    admin::attach(server);
    file::attach(server);
    status::attach(server);
    version::attach(server);
    versions::attach(server);
}
