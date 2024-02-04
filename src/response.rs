use tokio::fs::File;
use tokio::io::AsyncReadExt;

use crate::headers::Headers;
use crate::request::Request;
use crate::status::Status;

pub struct ResponseMeta {
    pub http_version: Vec<u8>,
    pub status: Status,
    pub headers: Headers,
}

impl ResponseMeta {
    pub fn from_req(request: Request, status: Status) -> Self {
        Self {
            http_version: request.http_version,
            status,
            headers: Headers::new(),
        }
    }

    pub fn ok(request: Request) -> Self {
        Self::from_req(request, Status::Ok)
    }

    pub fn not_found(request: Request) -> Self {
        Self::from_req(request, Status::NotFound)
    }

    pub fn empty(self) -> Response {
        Response {
            meta: self,
            data: None,
        }
    }

    pub fn plain(mut self, data: Vec<u8>) -> Response {
        self.headers
            .insert(b"Content-Type".to_vec(), b"text/plain".to_vec());
        self.headers.insert(
            b"Content-Length".to_vec(),
            format!("{}", data.len()).into_bytes(),
        );
        Response {
            meta: self,
            data: Some(data),
        }
    }

    pub async fn file(mut self, mut file: File) -> anyhow::Result<Response> {
        let mut data = Vec::new();
        file.read_to_end(&mut data).await?;

        self.headers.insert(
            b"Content-Type".to_vec(),
            b"application/octet-stream".to_vec(),
        );
        self.headers.insert(
            b"Content-Length".to_vec(),
            format!("{}", data.len()).into_bytes(),
        );
        Ok(Response {
            meta: self,
            data: Some(data),
        })
    }
}

pub struct Response {
    pub meta: ResponseMeta,
    pub data: Option<Vec<u8>>,
}
