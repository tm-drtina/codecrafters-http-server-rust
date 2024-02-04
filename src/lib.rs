use tokio::net::TcpStream;

mod headers;
mod request;
mod response;
mod tcp_http_reader;
mod tcp_http_writer;
mod server;

use crate::tcp_http_reader::TcpHttpReader;
use crate::server::create_response;
use crate::tcp_http_writer::TcpHttpWriter;

pub async fn process_stream(mut stream: TcpStream) -> anyhow::Result<()> {
    let (reader, writer) = stream.split();
    let mut reader = TcpHttpReader::new(reader);
    let mut writer = TcpHttpWriter::new(writer);

    // loop {
    let request = reader.read_request().await?;
    let response = create_response(request)?;
    writer.write_response(&response).await?;
    // }
    
    Ok(())
}
