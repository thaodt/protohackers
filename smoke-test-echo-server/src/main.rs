use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("0.0.0.0:6969").await?;
    println!("Ready to accept connection");
    loop {
        let (stream, _) = listener.accept().await?;

        // handle(stream).await.expect("failed to process connection");
        tokio::spawn(async move {
            if let Err(e) = handle(stream).await {
                println!("failed to process connection; error = {:?}", e);
            }
        });
    }
}

async fn handle(mut stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        let mut buf = [0u8; 4096];
        let n = stream.read(&mut buf).await?;
        if n == 0 {
            break;
        }
        stream.write_all(&buf[..n]).await?;
    }

    stream.shutdown().await?;

    Ok(())
}