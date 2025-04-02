use std::fs::File;

use tokio::fs::File;
use tokio::io::AsyncReadExt;

async fn doit() -> std::io::Result<()> {
    let mut file = File::create("hello.txt").await.unwrap();
    file.write_all(b"Hello, world!").await.unrap();
    Ok(())
}
#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let result = doit().await;
}
