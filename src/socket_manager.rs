// Dependencies:
use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use tungstenite::protocol::{Message as TungsteniteMessage};
use futures_util::{StreamExt, SinkExt};

pub async fn start_socket_server() {
    // Address to bind the server
    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(&addr).await.expect("Failed to bind");

    println!("WebSocket server started at: {}", addr);

    // Loop to accept incoming connections
    loop {
        match listener.accept().await {
            Ok((stream, _)) => {
                tokio::spawn(async move {
                    let ws_stream = accept_async(stream).await.expect("Error during the websocket handshake occurred");

                    // Split the stream into sender and receiver
                    let (mut ws_sender, mut ws_receiver) = ws_stream.split();

                    // Send a message to the client
                    let message = TungsteniteMessage::Text("Select seat class: FirstClass, BusinessClass, EconomyClass".to_string());
                    if ws_sender.send(message).await.is_err() {
                        eprintln!("Failed to send message");
                    }

                    // Receive a message from the client
                    if let Some(Ok(TungsteniteMessage::Text(response))) = ws_receiver.next().await {
                        println!("Client selected: {}", response);

                    }
                });
            },
            Err(e) => {
                eprintln!("Failed to accept connection: {}", e);
            }
        }
    }
}

