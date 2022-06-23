use afire::{Content, Request, Response};
use serde_json::json;
use sha2::{Digest, Sha256};

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

pub fn json_err(err: &str) -> Response {
    Response::new()
        .status(404)
        .content(Content::JSON)
        .text(json!({ "error": err }))
}

pub fn text_err_handle(err: &str, res_type: ResponseType) -> Response {
    match res_type {
        ResponseType::Text => Response::new().status(404).content(Content::TXT).text(err),
        ResponseType::Json => json_err(err),
    }
}

pub fn verify_password(req: &Request, real_hash: &str) -> Option<Response> {
    // Get password
    let password = match req.header("password") {
        Some(i) => i,
        None => return Some(json_err("No password")),
    };

    // Check password
    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    let hash = format!("{:x}", hasher.finalize());

    if hash != real_hash {
        return Some(json_err("Incorrect password"));
    }

    None
}
