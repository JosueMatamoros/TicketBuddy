// Dependencies:
use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use tungstenite::protocol::Message as TungsteniteMessage;

pub async fn start_socket_server() {
    // Address to bind the server
    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(&addr).await.expect("Failed to bind");

    println!("WebSocket server started at: \n{}", addr);

    // Loop to accept incoming connections
    loop {
        match listener.accept().await {
            Ok((stream, _)) => {
                tokio::spawn(async move {
                    let ws_stream = accept_async(stream)
                        .await
                        .expect("Error during the websocket handshake occurred");

                    // Split the stream into sender and receiver
                    let (mut ws_sender, mut ws_receiver) = ws_stream.split();

                    // Receive a message from the client
                    while let Some(Ok(TungsteniteMessage::Text(response))) =
                        ws_receiver.next().await
                    {
                        println!("Received: {}", response);
                        let response = response.as_str();
                        let section = section_match(response);

                        let formatted_sections: String = section
                            .iter()
                            .enumerate()
                            .map(|(i, sec)| format!("{}: {}", i + 1, sec))
                            .collect::<Vec<String>>()
                            .join("\n");

                        let sections_message = TungsteniteMessage::Text(format!(
                            "Available sections in {}:\n{}\nPlease select a section by number.",
                            response, formatted_sections
                        ));

                        if ws_sender.send(sections_message).await.is_err() {
                            eprintln!("Failed to send message");
                            break;
                        }
                    }
                });
            }
            Err(e) => {
                eprintln!("Failed to accept connection: {}", e);
            }
        }
    }
}

fn section_match(section: &str) -> Vec<&str> {
    match section {
        "FirstClass" => vec!["A1", "B1", "C1"],
        "BusinessClass" => vec!["A2", "B2", "C2", "A3", "B3", "C3"],
        "EconomyClass" => vec!["D", "E", "F", "G", "H"],
        _ => vec!["EconomyClass"],
    }
}
