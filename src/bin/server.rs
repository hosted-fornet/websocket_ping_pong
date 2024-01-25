use std::time::Instant;
use tokio::net::TcpListener;
use tokio_tungstenite::{accept_async, tungstenite::protocol::Message};
use futures_util::{stream::StreamExt, SinkExt};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:9000").await.unwrap();
    println!("WebSocket server listening on ws://localhost:9000");

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(handle_connection(stream));
    }
}

async fn handle_connection(stream: tokio::net::TcpStream) {
    let ws_stream = accept_async(stream).await.expect("Error during WebSocket handshake");
    let (mut write, mut read) = ws_stream.split();

    let mut trials = Vec::new();

    for _ in 0..100 {
        let start = Instant::now();
        write.send(Message::Ping(vec![])).await.unwrap();

        if let Some(message) = read.next().await {
            match message.expect("Failed to receive a message") {
                Message::Pong(_) => {
                    let duration = start.elapsed();
                    trials.push(duration);
                    //println!("Round-trip time: {:?}", duration);
                },
                Message::Close(_) => {
                    println!("Client requested to close the connection");
                    break;
                }
                _ => continue,
            }
        } else {
            break; // Exit if the stream ends
        }
    }

    let sum: std::time::Duration = trials.iter().sum();
    println!("Round-trip time avg: {:?} over {} trials", sum / trials.len() as u32, trials.len());

    // Send a close frame
    write.send(Message::Close(None)).await.unwrap();
    println!("Connection closed");
}
