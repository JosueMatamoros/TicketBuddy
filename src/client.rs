// bin/client.rs

// Dependencies:
use tokio_tungstenite::connect_async;
use tungstenite::protocol::Message;
use url::Url;
use futures_util::{SinkExt};
use std::io::{self, Write};

#[tokio::main]
async fn main() {
    // Service URL
    let url = Url::parse("ws://127.0.0.1:8080").unwrap();

    // Try to connect to the server
    let (mut ws_stream, _) = connect_async(url).await.expect("Failed to connect");

    println!("Connected to the server");

    // Show the user the available seat classes
    println!("Please select a seat class:");
    println!("1: FirstClass");
    println!("2: BusinessClass");
    println!("3: EconomyClass");

    // Get the user's input
    let mut input = String::new();
    io::stdout().flush().unwrap(); // Flush the buffer to show the message
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Match the user's input to the seat class
    let selected_class = match input.trim() {
        "1" => "FirstClass",
        "2" => "BusinessClass",
        "3" => "EconomyClass",
        _ => {
            println!("Invalid selection. Defaulting to EconomyClass.");
            "EconomyClass"
        }
    };

    // Send the selected seat class to the server
    let message = Message::Text(selected_class.to_string());
    ws_stream.send(message).await.expect("Failed to send message");

    println!("Sent: {}", selected_class);

    // Close the connection
    ws_stream.close(None).await.expect("Failed to close connection");
}
