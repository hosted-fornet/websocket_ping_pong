use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use futures_util::{SinkExt, StreamExt};

#[tokio::main]
async fn main() {
    let url = url::Url::parse("ws://localhost:9000").expect("Invalid URL");

    let (ws_stream, _) = match connect_async(url).await {
        Ok(stream) => {
            println!("WebSocket client connected");
            stream
        },
        Err(e) => {
            eprintln!("Failed to connect: {}", e);
            return;
        }
    };

    let (mut write, mut read) = ws_stream.split();

    while let Some(message) = read.next().await {
        match message {
            Ok(Message::Ping(ping_data)) => {
                if let Err(e) = write.send(Message::Pong(ping_data)).await {
                    eprintln!("Failed to send Pong: {}", e);
                    return;
                }
            },
            Ok(Message::Close(_)) => {
                println!("Server closed the connection");
                return;
            },
            Err(e) => {
                eprintln!("Error in receiving message: {}", e);
                return;
            }
            _ => {}
        }
    }
}
