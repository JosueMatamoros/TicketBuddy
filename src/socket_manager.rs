use crate::seat_manager::{find_seats_across_sections, mark_seat_as, Seat, Section};
use futures_util::{SinkExt, StreamExt};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use tungstenite::protocol::Message as TungsteniteMessage;

pub async fn start_socket_server(seats: Arc<Mutex<HashMap<(Section, u32, u32), Seat>>>) {
    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(&addr).await.expect("Failed to bind");

    println!("WebSocket server started at: \n{}", addr);

    loop {
        match listener.accept().await {
            Ok((stream, _)) => {
                let seats = Arc::clone(&seats);
                tokio::spawn(async move {
                    let ws_stream = accept_async(stream)
                        .await
                        .expect("Error during the websocket handshake occurred");

                    let (mut ws_sender, mut ws_receiver) = ws_stream.split();

                    // Wait for the client to send the class selection
                    if let Some(Ok(TungsteniteMessage::Text(class_selection))) =
                        ws_receiver.next().await
                    {
                        println!("Received class selection: {}", class_selection);

                        // Get the available sections for the selected class
                        let sections = section_match(&class_selection);

                        // Format the sections to send to the client
                        let formatted_sections: String = sections
                            .iter()
                            .enumerate()
                            .map(|(i, sec)| format!("{}: {}", i + 1, sec))
                            .collect::<Vec<String>>()
                            .join("\n");

                        // Create a message with the available sections
                        let sections_message = TungsteniteMessage::Text(format!(
                            "Available sections in {}:\n{}\nPlease select a section by number.",
                            class_selection, formatted_sections
                        ));

                        // Send the available sections to the client
                        if ws_sender.send(sections_message).await.is_err() {
                            eprintln!("Failed to send message");
                            return;
                        }

                        // Wait for the client to send the section selection
                        if let Some(Ok(TungsteniteMessage::Text(
                            section_selection_and_seat_count,
                        ))) = ws_receiver.next().await
                        {
                            println!(
                                "Received section selection: {}",
                                section_selection_and_seat_count
                            );

                            // Split the section selection and seat count
                            let parts: Vec<&str> =
                                section_selection_and_seat_count.split(';').collect();
                            if parts.len() == 2 {
                                let section_selection = parts[0];
                                let seat_count: u32 = parts[1].parse().unwrap_or(1);

                                    // Convert the section selection to the corresponding enum
                                let section_enum = section_string_to_enum(section_selection);

                                // Find the available seats across the selected sections
                                let available_seats = find_seats_across_sections(
                                    seat_count,
                                    section_enum,
                                    seats.clone(),
                                );

                                // Format the available seats to send to the client
                                let formatted_seats: Vec<String> = available_seats
                                    .iter()
                                    .map(|(section, row, number)| {
                                        format!("{:?}-{}-{}", section, row, number)
                                    })
                                    .collect();

                                let seat_suggestion_message = TungsteniteMessage::Text(format!(
                                    "{}",
                                    formatted_seats.join(", ")
                                ));

                                // Send the available seats to the client
                                if ws_sender.send(seat_suggestion_message).await.is_err() {
                                    eprintln!("Failed to send seat suggestion message");
                                }

                                // Wait for the client to send the confirmation
                                if let Some(Ok(TungsteniteMessage::Text(confirmation))) =
                                    ws_receiver.next().await
                                {
                                    match confirmation.trim() {
                                        "1" => {
                                            // If the client chooses 1, mark the seats as reserved ("B")
                                            for (section, row, number) in available_seats {
                                                mark_seat_as(
                                                    'B',
                                                    seats.clone(),
                                                    section,
                                                    row,
                                                    number,
                                                );
                                            }
                                            println!("Seats confirmed and marked as reserved.");
                                        }
                                        "2" => {
                                            // If the client chooses 2, mark the seats as declined ("F")
                                            for (section, row, number) in available_seats {
                                                mark_seat_as(
                                                    'F',
                                                    seats.clone(),
                                                    section,
                                                    row,
                                                    number,
                                                );
                                            }
                                            println!("Seats declined and marked as free.");
                                        }
                                        _ => {
                                            println!("Invalid selection received.");
                                        }
                                    }
                                }
                            }
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

// Function to match the class selection to the available sections
fn section_match(section: &str) -> Vec<&str> {
    match section {
        "FirstClass" => vec!["A1", "B1", "C1"],
        "BusinessClass" => vec!["A2", "B2", "C2", "A3", "B3", "C3"],
        "EconomyClass" => vec!["D", "E", "F", "G", "H"],
        _ => vec!["EconomyClass"], // Default value
    }
}

// Function to convert the section string to the corresponding enum
fn section_string_to_enum(section: &str) -> Section {
    match section {
        "A1" => Section::A1,
        "B1" => Section::B1,
        "C1" => Section::C1,
        "A2" => Section::A2,
        "B2" => Section::B2,
        "C2" => Section::C2,
        "A3" => Section::A3,
        "B3" => Section::B3,
        "C3" => Section::C3,
        "D" => Section::D,
        "E" => Section::E,
        "F" => Section::F,
        "G" => Section::G,
        "H" => Section::H,
        _ => Section::D, // Default value
    }
}
