use afire::Server;

use crate::App;

mod status;
mod version;
mod versions;

pub fn attach(server: &mut Server<App>) {
    status::attach(server);
    version::attach(server);
    versions::attach(server);
}
