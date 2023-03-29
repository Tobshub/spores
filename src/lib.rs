#[derive(Debug)]
pub struct Request {
    pub method: String,
    pub path: String,
    pub version: String,
}

impl Request {
    pub fn new(request_line: Vec<&str>) -> Request {
        Request {
            method: request_line[0].to_string(),
            path: request_line[1].to_string(),
            version: request_line[2].to_string(),
        }
    }
}
