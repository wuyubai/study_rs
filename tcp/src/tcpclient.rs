use std::env;

use bytes::Bytes;
use futures::{SinkExt, StreamExt};
use tokio_util::codec::{Framed, LengthDelimitedCodec};
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = env::args().nth(1).unwrap_or("127.0.0.1:8080".to_string());

    let socket = tokio::net::TcpStream::connect(addr).await?;
    let mut framed_stream = Framed::new(socket, LengthDelimitedCodec::new());

    framed_stream.send(Bytes::from("gettime")).await?;
    if let Some(msg) = framed_stream.next().await {
        match msg {
            Ok(msg) => {
                let timeinfo = String::from_utf8(msg.to_vec()).expect("Invalid UTF-8");
                println!("Time: {}", timeinfo);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }

    //    socket.write_all(b"gettime").await?;

    //     let mut buf: Vec<u8> = Vec::with_capacity(8128);
    //     let mut resp = [0u8; 2048];

    //     loop {
    //         let n = socket.read(&mut resp).await?;
    //         if n == 0 {
    //             panic!("Received 0 bytes");
    //         }
    //         buf.extend_from_slice(&resp[..n]);

    //         if buf.len() >= 10 {
    //             break;
    //         } else {
    //             continue;
    //         }
    //     }
    //     let timeinfo = String::from_utf8(buf)?;
    //     println!("Time: {}", timeinfo);

    Ok(())
}
