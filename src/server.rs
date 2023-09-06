use std::io;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
pub async fn init() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    println!("Server listening on port 8080");

    while let Ok((stream, _)) = listener.accept().await {
	tokio::spawn(handle_client(stream));
    }

    Ok(())
}

async fn handle_client(mut stream: TcpStream) -> Result<(), io::Error> {
    println!("Client connected: {:?}", stream.peer_addr());

    let mut buffer = [0; 1024];

    while let Ok(n) = stream.read(&mut buffer).await {
	if n == 0 {
	    break;
	}

	let message = String::from_utf8_lossy(&buffer[0..n]);
	println!("Received message: {}", message);
	
	let response = "Message received!";
	stream.write_all(response.as_bytes()).await?;
    }

    println!("Client disconnected: {:?}", stream.peer_addr());

    Ok(())
}
