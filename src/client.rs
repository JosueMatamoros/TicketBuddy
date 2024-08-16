// bin/client.rs

// Dependencies:
use futures_util::{SinkExt, StreamExt};
use std::io::{self, Write};
use tokio_tungstenite::connect_async;
use tungstenite::protocol::Message;
use url::Url;

#[tokio::main]
async fn main() {
    // Service URL
    let url = Url::parse("ws://127.0.0.1:8080").unwrap();

    // Try to connect to the server
    let (mut ws_stream, _) = connect_async(url).await.expect("Failed to connect");

    println!("Connected to the server");

    let link_text = "https://drive.google.com/file/d/1WbfwatCyM9QffUuCYRkS1nKkzq_9FBrT/view?usp=sharing";
    let clickable_link = format!("\x1b]8;;{}\x1b\\Click Here\x1b]8;;\x1b\\", link_text);
    println!("If you need more information about our stage distribution, {}", clickable_link);

    // Show the user the available seat classes
    println!("Please select a seat class:");
    println!("1: FirstClass");
    println!("2: BusinessClass");
    println!("3: EconomyClass");

    // Get the user's input
    let mut input = String::new();
    io::stdout().flush().unwrap(); // Flush the buffer to show the message
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

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
    ws_stream
        .send(message)
        .await
        .expect("Failed to send message");

    // Wait for the server to send the available sections
    if let Some(Ok(Message::Text(response))) = ws_stream.next().await {
        println!("{}", response);
    } else {
        println!("Failed to receive message");
    }

    let mut input = String::new();
    io::stdout().flush().unwrap(); // Flush the buffer to show the message
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // lamar a la fucnion
    let selected_section = match_section(selected_class, input.trim());

    // enviarla
    let message = Message::Text(selected_section.to_string());
    ws_stream
        .send(message)
        .await
        .expect("Failed to send message");

    // Wait for the server to send the available seats

    // Close the connection
    ws_stream
        .close(None)
        .await
        .expect("Failed to close connection");
}

fn match_section(class: &str, section: &str) -> &'static str {
    match class {
        "FirstClass" => match section {
            "1" => "A1",
            "2" => "B1",
            "3" => "C1",
            _ => "A1",
        },
        "BusinessClass" => match section {
            "1" => "A2",
            "2" => "B2",
            "3" => "C2",
            "4" => "A3",
            "5" => "B3",
            "6" => "C3",
            _ => "A2",
        },
        "EconomyClass" => match section {
            "1" => "D",
            "2" => "E",
            "3" => "F",
            "4" => "G",
            "5" => "H",
            _ => "D",
        },
        _ => "EconomyClass",
    }
}
