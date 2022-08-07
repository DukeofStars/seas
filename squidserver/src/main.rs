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

    // Read contents using ron format
    let mut contents = String::new();
    stream.read_to_string(&mut contents).await?;

    let msg: Message = ron::from_str(&contents)?;

    println!("{} sent: {:?}", addr, msg);

    Ok(())
}
