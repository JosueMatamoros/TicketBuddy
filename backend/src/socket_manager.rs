use crate::seat_manager::{find_seats_suggestions, mark_seat_as, Seat, Section};
use futures_util::{SinkExt, StreamExt};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use tungstenite::protocol::Message as TungsteniteMessage;

pub async fn start_socket_server(seats: Arc<Mutex<HashMap<(Section, u32, u32), Seat>>>) {
    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(&addr).await.expect("Failed to bind");

    println!("Servidor WebSocket iniciado en: {}", addr);

    loop {
        match listener.accept().await {
            Ok((stream, _)) => {
                let seats = Arc::clone(&seats);
                tokio::spawn(async move {
                    let ws_stream = accept_async(stream)
                        .await
                        .expect("Error durante el handshake WebSocket");

                    let (mut ws_sender, mut ws_receiver) = ws_stream.split();

                    // Esperar a que el cliente envíe el número de asientos
                    if let Some(Ok(TungsteniteMessage::Text(seat_count_str))) = ws_receiver.next().await {
                        println!("Número de asientos recibido: {}", seat_count_str);

                        // Parsear el número de asientos
                        let seat_count: u32 = seat_count_str.trim().parse().unwrap_or(1);

                        // Obtener las sugerencias de asientos
                        let seat_suggestions = find_seats_suggestions(seat_count, seats.clone());

                        // Formatear las sugerencias para enviarlas al cliente
                        let formatted_suggestions: Vec<String> = seat_suggestions.iter().enumerate().map(|(index, seats)| {
                            let seats_str = seats.iter()
                                .map(|(section, row, number)| format!("{:?}-Fila{}-Asiento{}", section, row, number))
                                .collect::<Vec<String>>()
                                .join(", ");
                            format!("Sugerencia {}: {}", index + 1, seats_str)
                        }).collect();

                        // Enviar las sugerencias al cliente
                        if ws_sender.send(TungsteniteMessage::Text(formatted_suggestions.join("|"))).await.is_err() {
                            eprintln!("Error al enviar las sugerencias al cliente");
                        }

                        // Manejar la confirmación o rechazo de las sugerencias por parte del cliente
                        // Esperar a que el cliente envíe su elección
                        if let Some(Ok(TungsteniteMessage::Text(choice_str))) = ws_receiver.next().await {
                            // Parsear la elección del cliente
                            // Se espera que el cliente envíe el número de la sugerencia aceptada o '0' si rechaza todas
                            let choice: usize = choice_str.trim().parse().unwrap_or(0);

                            if choice >= 1 && choice <= seat_suggestions.len() {
                                // El cliente ha aceptado una de las sugerencias
                                let accepted_seats = &seat_suggestions[choice - 1];

                                // Marcar los asientos aceptados como reservados ('B')
                                for &(section, row, number) in accepted_seats {
                                    mark_seat_as('B', seats.clone(), section, row, number);
                                }

                                // Marcar los asientos de las otras sugerencias como disponibles ('F')
                                for (i, suggestion) in seat_suggestions.iter().enumerate() {
                                    if i != (choice - 1) {
                                        for &(section, row, number) in suggestion {
                                            mark_seat_as('F', seats.clone(), section, row, number);
                                        }
                                    }
                                }

                                println!("El cliente ha aceptado la sugerencia {}", choice);
                                // Enviar confirmación al cliente
                                if ws_sender.send(TungsteniteMessage::Text("Reserva confirmada".to_string())).await.is_err() {
                                    eprintln!("Error al enviar la confirmación al cliente");
                                }
                            } else {
                                // El cliente ha rechazado todas las sugerencias
                                // Marcar todos los asientos como disponibles ('F')
                                for suggestion in seat_suggestions {
                                    for &(section, row, number) in &suggestion {
                                        mark_seat_as('F', seats.clone(), section, row, number);
                                    }
                                }

                                println!("El cliente ha rechazado todas las sugerencias");
                                // Enviar notificación al cliente
                                if ws_sender.send(TungsteniteMessage::Text("Reserva rechazada".to_string())).await.is_err() {
                                    eprintln!("Error al enviar la notificación al cliente");
                                }
                            }
                        }
                    }
                });
            }
            Err(e) => {
                eprintln!("Error al aceptar conexión: {}", e);
            }
        }
    }
}
