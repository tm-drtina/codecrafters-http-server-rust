use anyhow::{anyhow, Context};
use tokio::fs::File;

use crate::request::Request;
use crate::response::{Response, ResponseMeta};
use crate::Config;

pub async fn create_response(request: Request, config: &Config) -> anyhow::Result<Response> {
    Ok(
        match (request.method.as_slice(), request.target.as_slice()) {
            (b"GET", b"/") => ResponseMeta::ok(request).empty(),
            (b"GET", b"/user-agent") => {
                let data = request
                    .headers
                    .get(&b"User-Agent".to_vec())
                    .ok_or(anyhow!("User-Agent header not set!"))?
                    .clone();
                ResponseMeta::ok(request).plain(data)
            }
            (b"GET", target) if target.starts_with(b"/echo/") => {
                let data = target.strip_prefix(b"/echo/").unwrap().to_vec();
                ResponseMeta::ok(request).plain(data)
            }
            (b"GET", target) if target.starts_with(b"/files/") => {
                let filename = std::str::from_utf8(target.strip_prefix(b"/files/").unwrap()).context("Cannot parse filename to string")?;
                let path = config.directory.join(filename);
                if path.exists() {
                    let file = File::open(path).await.context("Cannot open file")?;
                    ResponseMeta::ok(request).file(file).await?
                } else {
                    ResponseMeta::not_found(request).empty()
                }
            }
            (_, _) => ResponseMeta::not_found(request).empty(),
        },
    )
}
