use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use tungstenite::protocol::Message as TungsteniteMessage;
use futures_util::{StreamExt, SinkExt};
use crate::seat_manager::{Section, Seat};

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

                        // Esperamos la selección de sección del cliente
                        if let Some(Ok(TungsteniteMessage::Text(section_selection))) = ws_receiver.next().await {
                            println!("Received section selection: {}", section_selection);

                            // Aquí puedes manejar la selección de la sección si es necesario.
                            // Por ahora, solo confirmamos la recepción.
                            let confirmation_message = TungsteniteMessage::Text(format!(
                                "Section {} selected. Waiting for further instructions.",
                                section_selection
                            ));

                            if ws_sender.send(confirmation_message).await.is_err() {
                                eprintln!("Failed to send message");
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
