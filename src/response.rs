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

impl Response {
    pub fn empty(self) -> ResponseWithData {
        ResponseWithData {
            status: self.status,
            headers: self.headers,
            data: None,
        }
    }

    pub fn plain(mut self, data: Vec<u8>) -> ResponseWithData {
        self.headers
            .insert(b"Content-Type".to_vec(), b"text/plain".to_vec());
        self.headers.insert(
            b"Content-Length".to_vec(),
            format!("{}", data.len()).into_bytes(),
        );
        ResponseWithData {
            status: self.status,
            headers: self.headers,
            data: Some(data),
        }
    }
}

pub struct ResponseWithData {
    pub status: StatusLine,
    pub headers: Headers,
    pub data: Option<Vec<u8>>,
}
