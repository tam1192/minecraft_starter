use std::error::Error;

use log::{debug, error, info};
use simple_logger::SimpleLogger;
use tokio::{
    io::AsyncReadExt,
    net::{TcpListener, TcpStream},
};

async fn in_coming(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    //let mut data = String::new();
    let mut data = Vec::new();
    //let _size = stream.read_to_string(&mut data)?;
    let _size = stream.read_to_end(&mut data).await?;
    debug!("connected {}: {:?}", stream.peer_addr()?, data);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    SimpleLogger::new().env().init().unwrap();
    let listener = TcpListener::bind("0.0.0.0:56562").await?;
    info!("server startup!");
    loop {
        if let Ok((stream, _)) = listener.accept().await {
            if let Err(e) = in_coming(stream).await {
                error!("{}", e);
            }
        }
    }
    Ok(())
}
