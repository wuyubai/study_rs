use std::env;

use bytes::Bytes;
use futures::{SinkExt, StreamExt};
use tokio::process::Command;
use tokio_util::codec::{Framed, LengthDelimitedCodec};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = env::args().nth(1).unwrap_or("0.0.0.0:8080".to_string());

    println!("Listening on: {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;

    loop {
        let (socket, _) = listener.accept().await?;
        let mut framed_stream = Framed::new(socket, LengthDelimitedCodec::new());

        tokio::spawn(async move {
            while let Some(msg) = framed_stream.next().await {
                match msg {
                    Ok(msg) => {
                        let directive = String::from_utf8(msg.to_vec()).expect("Invalid UTF-8");
                        println!("Received: {}", directive);
                        let output = process(&directive).await;
                        println!("Output: {}", output);
                        _ = framed_stream.send(Bytes::from(output)).await;
                    }
                    Err(e) => {
                        println!("Error: {}", e);
                    }
                }
            }
            // let mut buf = [0; 1024];
            // let mut offset = 0;
            // loop {
            //     let n = socket
            //         .read(&mut buf[offset..])
            //         .await
            //         .expect("read data failed");
            //     if n == 0 {
            //         return;
            //     }
            //     println!("Received {} bytes", n);
            //     let end = offset + n;
            //     if let Ok(directive) = std::str::from_utf8(&buf[..end]) {
            //         println!("Received: {}", directive);
            //         let output = process(directive).await;
            //         println!("Output: {}", output);
            //         socket
            //             .write_all(output.as_bytes())
            //             .await
            //             .expect("write failed");
            //     } else {
            //         offset = end;
            //     }
            // }
        });
    }
}

async fn process(directive: &str) -> String {
    if directive == "gettime" {
        let output = Command::new("cmd.exe")
            .arg("/c")
            .arg("date /t")
            .output()
            .await
            .unwrap();
        // String::from_utf8(output.stdout).unwrap()
        "2021-09-01".to_string()
    } else {
        "Invalid command".to_string()
    }
}
