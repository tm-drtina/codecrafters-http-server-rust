use crate::headers::Headers;

#[derive(Debug)]
pub struct Request {
    pub method: Vec<u8>,
    pub target: Vec<u8>,
    pub http_version: Vec<u8>,
    pub headers: Headers,
}
