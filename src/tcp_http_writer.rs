use tokio::io::AsyncWriteExt;
use tokio::net::tcp::WriteHalf;

use crate::response::Response;

pub struct TcpHttpWriter<'a> {
    writer: WriteHalf<'a>,
}

impl<'a> TcpHttpWriter<'a> {
    pub(crate) fn new(writer: WriteHalf<'a>) -> Self {
        Self { writer }
    }

    pub async fn write_response(&mut self, response: &Response) -> anyhow::Result<()> {
        let Response { meta, data } = response;
        // Response line
        self.writer.write_all(&meta.http_version).await?;
        self.writer.write_u8(b' ').await?;
        self.writer.write_all(meta.status.repr()).await?;
        self.writer.write_u8(b'\r').await?;
        self.writer.write_u8(b'\n').await?;

        for (key, value) in &meta.headers {
            self.writer.write_all(key).await?;
            self.writer.write_u8(b':').await?;
            self.writer.write_all(value).await?;
            self.writer.write_u8(b'\r').await?;
            self.writer.write_u8(b'\n').await?;
        }

        // End of header
        self.writer.write_u8(b'\r').await?;
        self.writer.write_u8(b'\n').await?;

        // Data if needed
        if let Some(data) = data {
            self.writer.write_all(data).await?;
        }

        Ok(())
    }
}
