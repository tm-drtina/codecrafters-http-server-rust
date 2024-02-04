use crate::headers::Headers;

pub struct StatusLine {
    pub http_version: Vec<u8>,
    pub status_code: u16,
    pub status_text: Vec<u8>,
}

pub struct Response {
    pub status: StatusLine,
    pub headers: Headers,
}
