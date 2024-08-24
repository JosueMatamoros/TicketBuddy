use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use tungstenite::protocol::Message as TungsteniteMessage;
use futures_util::{StreamExt, SinkExt};
use crate::seat_manager::{Section, Seat, find_seats_by_section, mark_seat_as};

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

                    // Esperamos la selección de clase de asiento del cliente
                    if let Some(Ok(TungsteniteMessage::Text(class_selection))) = ws_receiver.next().await {
                        println!("Received class selection: {}", class_selection);

                        // Usamos section_match para obtener las secciones correspondientes a la clase seleccionada
                        let sections = section_match(&class_selection);

                        // Formateamos las secciones para enviarlas al cliente
                        let formatted_sections: String = sections
                            .iter()
                            .enumerate()
                            .map(|(i, sec)| format!("{}: {}", i + 1, sec))
                            .collect::<Vec<String>>()
                            .join("\n");

                        let sections_message = TungsteniteMessage::Text(format!(
                            "Available sections in {}:\n{}\nPlease select a section by number.",
                            class_selection, formatted_sections
                        ));

                        // Enviamos las secciones disponibles al cliente
                        if ws_sender.send(sections_message).await.is_err() {
                            eprintln!("Failed to send message");
                            return;
                        }

                        // Esperamos la selección de sección y cantidad de asientos del cliente
                        if let Some(Ok(TungsteniteMessage::Text(section_selection_and_seat_count))) = ws_receiver.next().await {
                            println!("Received section selection: {}", section_selection_and_seat_count);

                            // Dividimos el mensaje en sección y cantidad de asientos
                            let parts: Vec<&str> = section_selection_and_seat_count.split(';').collect();
                            if parts.len() == 2 {
                                let section_selection = parts[0];
                                let seat_count: u32 = parts[1].parse().unwrap_or(1);

                                // Convertir la selección de la sección a su equivalente en el enum Section
                                let section_enum = section_string_to_enum(section_selection);

                                // Llamamos a la función que encuentra los asientos disponibles
                                let available_seats = find_seats_by_section(seat_count, section_enum, seats.clone());

                                // Formateamos los asientos para enviarlos al cliente
                                let formatted_seats: Vec<String> = available_seats
                                    .iter()
                                    .map(|(section, row, number)| format!("{:?} - {} - {}", section, row, number))
                                    .collect();

                                let seat_suggestion_message = TungsteniteMessage::Text(format!(
                                    "Suggested seats: {}",
                                    formatted_seats.join(", ")
                                ));

                                // Enviar las sugerencias de asientos al cliente
                                if ws_sender.send(seat_suggestion_message).await.is_err() {
                                    eprintln!("Failed to send seat suggestion message");
                                }

                                // Esperar la confirmación del cliente
                                if let Some(Ok(TungsteniteMessage::Text(confirmation))) = ws_receiver.next().await {
                                    if confirmation.trim().eq_ignore_ascii_case("yes") {
                                        // Si el cliente acepta la sugerencia, marcamos los asientos como reservados ("B")
                                        for (section, row, number) in available_seats {
                                            mark_seat_as('F', seats.clone(), section, row, number);
                                        }
                                        println!("Seats declined and marked as free.");
                                    } else {
                                        // Si el cliente rechaza la sugerencia, marcamos los asientos como disponibles ("F")
                                        for (section, row, number) in available_seats {
                                            mark_seat_as('B', seats.clone(), section, row, number);
                                        }
                                        println!("Seats confirmed and marked as reserved.");
                                    }
                                }
                            } else {
                                eprintln!("Invalid section selection and seat count received.");
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

// Función para mapear la selección de clase a las secciones disponibles
fn section_match(section: &str) -> Vec<&str> {
    match section {
        "FirstClass" => vec!["A1", "B1", "C1"],
        "BusinessClass" => vec!["A2", "B2", "C2", "A3", "B3", "C3"],
        "EconomyClass" => vec!["D", "E", "F", "G", "H"],
        _ => vec!["EconomyClass"], // Valor predeterminado si la clase no coincide
    }
}

// Función para convertir el nombre de una sección a su correspondiente enum
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
        _ => Section::D, // Valor predeterminado
    }
}
