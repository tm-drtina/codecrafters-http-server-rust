use crate::headers::Headers;
use crate::request::Request;
use crate::response::{Response, ResponseWithData, StatusLine};

pub fn create_response(request: Request) -> ResponseWithData {
    match (
        request.request_line.method.as_slice(),
        request.request_line.target.as_slice(),
    ) {
        (b"GET", b"/") => Response {
            status: StatusLine {
                http_version: request.request_line.http_version,
                status_code: 200,
                status_text: b"OK".to_vec(),
            },
            headers: Headers::new(),
        }.empty(),
        (b"GET", target) if target.starts_with(b"/echo/") => Response {
            status: StatusLine {
                http_version: request.request_line.http_version,
                status_code: 200,
                status_text: b"OK".to_vec(),
            },
            headers: Headers::new(),
        }.plain(target.strip_prefix(b"/echo/").unwrap().to_vec()),
        (_, _) => Response {
            status: StatusLine {
                http_version: request.request_line.http_version,
                status_code: 404,
                status_text: b"Not Found".to_vec(),
            },
            headers: Headers::new(),
        }.empty(),
    }
}
