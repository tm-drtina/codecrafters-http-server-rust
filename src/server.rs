use crate::headers::Headers;
use crate::request::Request;
use crate::response::{Response, StatusLine};

pub fn create_response(request: Request) -> Response {
    if request.request_line.target == b"/" {
        Response {
            status: StatusLine {
                http_version: request.request_line.http_version,
                status_code: 200,
                status_text: b"OK".to_vec(),
            },
            headers: Headers::new(),
        }
    } else {
        Response {
            status: StatusLine {
                http_version: request.request_line.http_version,
                status_code: 404,
                status_text: b"Not Found".to_vec(),
            },
            headers: Headers::new(),
        }
    }
}
