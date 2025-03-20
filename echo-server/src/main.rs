use std::env;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read port from environment variable, panic if not set
    let port = env::var("PORT").expect("PORT environment variable must be set");
    let addr = format!("0.0.0.0:{}", port);
    
    let listener = TcpListener::bind(&addr).await?;
    println!("Echo server listening on {}", addr);

    loop {
        let (mut socket, addr) = listener.accept().await?;
        println!("New connection from: {}", addr);

        tokio::spawn(async move {
            let mut buf = vec![0; 1024];

            loop {
                match socket.read(&mut buf).await {
                    Ok(0) => {
                        println!("Connection closed by client: {}", addr);
                        return;
                    }
                    Ok(n) => {
                        if let Err(e) = socket.write_all(&buf[..n]).await {
                            println!("Failed to write to socket: {}", e);
                            return;
                        }
                    }
                    Err(e) => {
                        println!("Failed to read from socket: {}", e);
                        return;
                    }
                }
            }
        });
    }
}