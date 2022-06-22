use afire::{Content, Request, Response};
use serde_json::json;

pub enum ResponseType {
    Json,
    Text,
}

impl ResponseType {
    pub fn from_headers(req: &Request) -> Self {
        if let Some(i) = req.header("Accept") {
            if i == "text/plain" {
                return ResponseType::Text;
            }
        }

        ResponseType::Json
    }
}

pub fn text_err_handle(err: &str, res_type: ResponseType) -> Response {
    match res_type {
        ResponseType::Text => Response::new().status(404).content(Content::TXT).text(err),
        ResponseType::Json => Response::new()
            .status(404)
            .content(Content::JSON)
            .text(json!({ "error": err })),
    }
}
