use afire::Request;

pub enum ResponseType {
    Json,
    Text
}

impl ResponseType {
    pub fn fromHeaders(req: &Request) -> Self {
        if let Some(i) = req.header("Accept") {
            if i == "text/plain" {
                return ResponseType::Text;
            }
        }
        
        ResponseType::Json
    }
}