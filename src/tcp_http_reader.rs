use anyhow::{anyhow, Context};
use nom::InputIter;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, BufReader};
use tokio::net::tcp::ReadHalf;

use crate::headers::Headers;
use crate::request::Request;

pub struct TcpHttpReader<'a> {
    reader: BufReader<ReadHalf<'a>>,
    buf: Vec<u8>,
}

impl<'a> TcpHttpReader<'a> {
    pub(crate) fn new(reader: ReadHalf<'a>) -> Self {
        Self {
            reader: BufReader::new(reader),
            buf: Vec::new(),
        }
    }

    async fn read_until_crlf(&mut self) -> std::io::Result<usize> {
        let mut read = 0;
        loop {
            read += self.reader.read_until(b'\n', &mut self.buf).await?;
            if read > 1
                && self.buf[self.buf.len() - 1] == b'\n'
                && self.buf[self.buf.len() - 2] == b'\r'
            {
                // Remove the ending CRLF
                self.buf.pop();
                self.buf.pop();
                read -= 2;
                break;
            }
        }

        Ok(read)
    }

    pub async fn read_request(&mut self) -> anyhow::Result<Request> {
        self.buf.clear();
        self.read_until_crlf().await?;
        let line = self.buf.as_slice();

        let (method, line) = line.split_at(
            line.position(|ch| ch == b' ')
                .ok_or(anyhow!("Cannot extract method from request line."))?,
        );
        let line = &line[1..];
        let (target, http_version) = line.split_at(line.position(|ch| ch == b' ').ok_or(
            anyhow!("Cannot request target and HTTP version from request line."),
        )?);
        let http_version = &http_version[1..];

        debug_assert_eq!(http_version, b"HTTP/1.1");

        let method = method.to_vec();
        let target = target.to_vec();
        let http_version = http_version.to_vec();

        let mut headers = Headers::new();
        loop {
            self.buf.clear();
            let read = self.read_until_crlf().await?;
            if read == 0 {
                break;
            }
            let line = self.buf.as_slice();
            let (key, value) = line.split_at(
                line.position(|ch| ch == b':')
                    .ok_or(anyhow!("Invalid header format"))?,
            );
            let mut value = &value[1..];
            while value.starts_with(b" ") {
                value = &value[1..];
            }
            headers.insert(key.to_vec(), value.to_vec());
        }

        let data = if let Some(data_len) = headers.get(&b"Content-Length".to_vec()) {
            let data_len = std::str::from_utf8(data_len)
                .context("Content-Length header valus is not valid utf-8 string")?
                .parse::<usize>()
                .context("Content-Length header is not a valid number")?;

            let mut buf = Vec::with_capacity(data_len);
            buf.resize(data_len, 0);
            self.reader.read_exact(&mut buf[..data_len]).await?;
            Some(buf.into_boxed_slice())
        } else {
            None
        };

        Ok(Request {
            method,
            target,
            http_version,
            headers,
            data,
        })
    }
}
