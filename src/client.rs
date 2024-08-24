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
    println!("1: FirstClass (Max 5 seats)");
    println!("2: BusinessClass (Max 6 seats)");
    println!("3: EconomyClass (Max 8 seats)");

    // Get the user's input for seat class
    let mut input = String::new();
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // Match the user's input to the seat class and determine the maximum seats allowed
    let (selected_class, max_seats) = match input.trim() {
        "1" => ("FirstClass", 5),
        "2" => ("BusinessClass", 6),
        "3" => ("EconomyClass", 8),
        _ => {
            println!("Invalid selection. Defaulting to EconomyClass.");
            ("EconomyClass", 8)
        }
    };

    // Send the selected seat class to the server
    ws_stream
        .send(Message::Text(selected_class.to_string()))
        .await
        .expect("Failed to send message");

    // Wait for the server to send the available sections
    if let Some(Ok(Message::Text(response))) = ws_stream.next().await {
        println!("{}", response);
    } else {
        println!("Failed to receive message");
    }

    // Get the user's section choice
    let mut input = String::new();
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // Match the selected section based on the selected class
    let selected_section = match_section(selected_class, input.trim());

    // Ask the user how many seats they want to reserve, constrained by the max allowed
    let num_seats = loop {
        println!("You can reserve up to {} seats in this section. How many seats would you like to reserve?", max_seats);
        let mut input = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim().parse::<u32>() {
            Ok(n) if n <= max_seats => break n,
            _ => println!("Invalid number of seats. Please enter a number between 1 and {}", max_seats),
        }
    };

    // Send the selected section and the number of seats to the server
    let message = Message::Text(format!("{};{}", selected_section, num_seats));
    ws_stream
        .send(message)
        .await
        .expect("Failed to send message");

    // Wait for the server to send the actual available seats
    if let Some(Ok(Message::Text(suggested_seats))) = ws_stream.next().await {
        println!("{}", suggested_seats);
    } else {
        println!("Failed to receive seat suggestions");
    }

    // Ask the user if they want to accept the suggestion
    println!("Do you want to accept this suggestion? (yes/no)");
    let mut input = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    if input.trim().eq_ignore_ascii_case("yes") {
        ws_stream
            .send(Message::Text("ACCEPT".to_string()))
            .await
            .expect("Failed to send acceptance message");
        println!("Reservation accepted.");
    } else {
        println!("Reservation declined.");
    }

    // Close the connection
    ws_stream
        .close(None)
        .await
        .expect("Failed to close connection");
}

// Helper function to match the selected section based on class and input
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
        _ => "D", // Default to a valid section in case of an invalid input
    }
}
