use std::fs;
use std::env;
use serde::Deserialize;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[derive(Deserialize)]
struct Config {
    port: u16,
}

fn read_config(config_path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let config_str = fs::read_to_string(config_path)?;
    let config: Config = serde_json::from_str(&config_str)?;
    Ok(config)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get config file path from command line argument or use default
    let args: Vec<String> = env::args().collect();
    let config_path = if args.len() > 1 {
        &args[1]
    } else {
        panic!("Error: Config file path must be provided as an argument")
    };
    
    println!("Reading configuration from: {}", config_path);
    let config = read_config(config_path)?;
    let addr = format!("0.0.0.0:{}", config.port);
    
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