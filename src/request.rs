use crate::headers::Headers;

#[derive(Debug)]
pub struct RequestLine {
    pub method: Vec<u8>,
    pub target: Vec<u8>,
    pub http_version: Vec<u8>,
}

#[derive(Debug)]
pub struct Request {
    pub request_line: RequestLine,
    pub headers: Headers,
}
