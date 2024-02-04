use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;

use anyhow::{bail, Context as _};
use tokio::net::TcpListener;

use http_server_starter_rust::{process_stream,Config};

fn create_config() -> anyhow::Result<Config> {
    let mut args = std::env::args();
    args.next().context("Expected first arg (path of executable)")?;

    Ok(match args.len() {
        0 => {
            eprintln!("No directory provided! Assuming `./data`");
            Config {
                directory: PathBuf::from_str("./data").unwrap(),
            }
        },
        2 => {
            let key = args.next().unwrap();
            let value = args.next().unwrap();
            anyhow::ensure!(key == "--directory", "Unrecognized argument. Expected '--directory'");
            Config {
                directory: PathBuf::from(value),
            }
        }
        _ => {
            bail!("Invalid number of arguments")
        }
    })
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:4221").await?;
    let config = Arc::new(create_config()?);
    
    loop {
        match listener.accept().await {
            Ok((stream, addr)) => {
                let config = Arc::clone(&config);
                tokio::spawn(async move {
                    eprintln!("new client: {:?}", addr);
                    match process_stream(stream, config).await {
                        Ok(_) => {},
                        Err(err) => eprintln!("Processing of stream failed: {}", err),
                    };
                });
            }
            Err(e) => eprintln!("couldn't get client: {:?}", e),
        }
    }
}
