use std::{error::Error, io, net::SocketAddr};

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
    // Send hello
    println!("New connection from {}", addr);

    // Obtain header (size of the message)
    let header: u16 = stream.read_u16().await?;

    // Read contents
    let mut buf = Vec::with_capacity(header as usize);
    stream.read_exact(&mut buf).await?;
    let msg: MessageHeader = ron::from_str(&String::from_utf8(buf.to_vec())?)?;

    println!("{} sent: {:?}", addr, msg);

    Ok(())
}
