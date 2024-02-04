use std::path::PathBuf;
use std::sync::Arc;

use tokio::net::TcpStream;

mod headers;
mod request;
mod response;
mod tcp_http_reader;
mod tcp_http_writer;
mod server;
mod status;

use crate::tcp_http_reader::TcpHttpReader;
use crate::server::create_response;
use crate::tcp_http_writer::TcpHttpWriter;

pub struct Config {
    pub directory: PathBuf,
}

pub async fn process_stream(mut stream: TcpStream, config: Arc<Config>) -> anyhow::Result<()> {
    let (reader, writer) = stream.split();
    let mut reader = TcpHttpReader::new(reader);
    let mut writer = TcpHttpWriter::new(writer);

    // loop {
    let request = reader.read_request().await?;
    let response = create_response(request, &config).await?;
    writer.write_response(&response).await?;
    // }
    
    Ok(())
}
