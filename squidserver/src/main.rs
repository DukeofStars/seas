use std::{error::Error, io, net::SocketAddr};

use serde::Deserializer;
use tokio::{
    io::AsyncReadExt,
    net::{TcpListener, TcpStream},
};

use squidserver::*;

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    println!("Starting server");

    let listener = TcpListener::bind("127.0.0.1:5336").await?;

    // connection handler
    loop {
        match listener.accept().await {
            Ok((stream, addr)) => {
                on_connect(stream, addr).await.unwrap();
            }
            Err(err) => return Err(err),
        }
    }
}

async fn on_connect(mut stream: TcpStream, addr: SocketAddr) -> Result<(), Box<dyn Error>> {
    println!("New connection from {}", addr);

    // Obtain header (size of the message)
    let header: u16 = stream.read_u16_le().await?;
    println!("Header size: {header}");

    // Read contents
    let mut buf = Vec::with_capacity(header as usize);
    stream.read_exact(&mut buf).await?;
    println!("Read data");
    let string = String::from_utf8(buf.to_vec())?;
    println!("Read utf8 string '{}'", string);
    let msg: Message = Message::Hello(1);
    println!("Serialised message");

    println!("{} sent: {:?}", addr, msg);

    Ok(())
}
