use std::fs;

use afire::{Content, Method, Response, Server};

use crate::App;

pub fn attach(server: &mut Server<App>) {
    server.route(Method::GET, "/admin/edit/*", |_req| {
        Response::new()
            .content(Content::HTML)
            .bytes(fs::read("web/serve/edit.html").unwrap())
    })
}
