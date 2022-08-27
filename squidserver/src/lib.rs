use serde::{Deserialize, Serialize};

use tokio::{io::AsyncWriteExt, net::TcpStream};

use std::error::Error;
use std::mem::size_of_val;

#[derive(Debug, Deserialize, Serialize)]
pub enum Message {
    Hello(u16),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MessageHeader {
    pub size: u16,
}

/// Sends a message to the local running daemon.
pub async fn send(msg: Message) -> Result<TcpStream, Box<dyn Error>> {
    dbg!(&msg);

    let size: u16 = size_of_val(&msg) as u16;
    println!("Size: {size}");

    let mut stream = TcpStream::connect("127.0.0.1:5336").await?;
    stream.write_all(&size.to_le_bytes()).await?;
    stream.write_all(ron::to_string(&msg)?.as_bytes()).await?;
    stream.flush().await?;

    Ok(stream)
}
