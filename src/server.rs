use anyhow::anyhow;

use crate::headers::Headers;
use crate::request::Request;
use crate::response::{Response, ResponseWithData, StatusLine};

pub fn create_response(request: Request) -> anyhow::Result<ResponseWithData> {
    Ok(
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
            }
            .empty(),
            (b"GET", b"/user-agent") => Response {
                status: StatusLine {
                    http_version: request.request_line.http_version,
                    status_code: 200,
                    status_text: b"OK".to_vec(),
                },
                headers: Headers::new(),
            }
            .plain(
                request
                    .headers
                    .get(&b"User-Agent".to_vec())
                    .ok_or(anyhow!("User-Agent header not set!"))?
                    .clone(),
            ),
            (b"GET", target) if target.starts_with(b"/echo/") => Response {
                status: StatusLine {
                    http_version: request.request_line.http_version,
                    status_code: 200,
                    status_text: b"OK".to_vec(),
                },
                headers: Headers::new(),
            }
            .plain(target.strip_prefix(b"/echo/").unwrap().to_vec()),
            (_, _) => Response {
                status: StatusLine {
                    http_version: request.request_line.http_version,
                    status_code: 404,
                    status_text: b"Not Found".to_vec(),
                },
                headers: Headers::new(),
            }
            .empty(),
        },
    )
}
