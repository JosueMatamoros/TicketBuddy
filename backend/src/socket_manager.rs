// socket_manager.rs

use crate::seat_manager::{find_seats_suggestions_by_category, get_seat_states, mark_seat_as, Seat, Category, Section};
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize};
use serde_json;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use tungstenite::protocol::Message as TungsteniteMessage;

#[derive(Debug, Deserialize)]
struct SeatRequest {
    category: Category,
    seat_count: u32,
}

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

                    // Enviar el estado actual de los asientos al cliente al conectarse
                    let seat_states = get_seat_states(seats.clone());
                    let seat_states_json = serde_json::to_string(&seat_states).unwrap();

                    if ws_sender
                        .send(TungsteniteMessage::Text(seat_states_json))
                        .await
                        .is_err()
                    {
                        eprintln!("Error al enviar el estado de los asientos al cliente");
                    }

                    // Esperar a que el cliente envíe la categoría y el número de asientos
                    if let Some(Ok(TungsteniteMessage::Text(request_str))) = ws_receiver.next().await {
                        println!("Solicitud recibida: {}", request_str);

                        // Parsear el mensaje del cliente
                        let seat_request: SeatRequest = match serde_json::from_str(&request_str) {
                            Ok(request) => request,
                            Err(e) => {
                                eprintln!("Error al deserializar SeatRequest: {}", e);
                                SeatRequest {
                                    category: Category::Economy,
                                    seat_count: 1,
                                }
                            }
                        };

                        // Obtener las sugerencias de asientos
                        let seat_suggestions = find_seats_suggestions_by_category(
                            seat_request.seat_count,
                            seat_request.category,
                            seats.clone(),
                        );

                        println!("Sugerencias encontradas: {:?}", seat_suggestions);

                        // Marcar los asientos sugeridos como reservados temporalmente ('R')
                        for suggestion in &seat_suggestions {
                            for &(section, row, number) in suggestion {
                                mark_seat_as('R', seats.clone(), section, row, number);
                            }
                        }

                        // Formatear las sugerencias para enviarlas al cliente
                        let formatted_suggestions: Vec<String> = seat_suggestions.iter().enumerate().map(|(index, seats)| {
                            let seats_str = seats.iter()
                                .map(|(section, row, number)| format!("{:?}-Fila{}-Asiento{}", section, row, number))
                                .collect::<Vec<String>>()
                                .join(", ");
                            format!("Sugerencia {}: {}", index + 1, seats_str)
                        }).collect();

                        println!("Sugerencias formateadas: {:?}", formatted_suggestions);

                        // Enviar las sugerencias al cliente
                        if !formatted_suggestions.is_empty() {
                            println!("Enviando sugerencias al cliente...");
                            if ws_sender.send(TungsteniteMessage::Text(formatted_suggestions.join("|"))).await.is_err() {
                                eprintln!("Error al enviar las sugerencias al cliente");
                            }
                        } else {
                            // No hay sugerencias disponibles
                            println!("No hay suficientes asientos disponibles en la categoría solicitada");
                            if ws_sender.send(TungsteniteMessage::Text("No hay suficientes asientos disponibles en la categoría solicitada".to_string())).await.is_err() {
                                eprintln!("Error al enviar el mensaje al cliente");
                            }
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

                                // Los asientos aceptados permanecen como 'R' (reservado temporalmente)
                                // Los asientos de las otras sugerencias se marcan como 'F' (disponible)
                                for (i, suggestion) in seat_suggestions.iter().enumerate() {
                                    if i != (choice - 1) {
                                        for &(section, row, number) in suggestion {
                                            mark_seat_as('F', seats.clone(), section, row, number);
                                        }
                                    }
                                }

                                println!("El cliente ha aceptado la sugerencia {}", choice);
                                // Enviar confirmación al cliente
                                if ws_sender.send(TungsteniteMessage::Text("Sugerencia aceptada".to_string())).await.is_err() {
                                    eprintln!("Error al enviar la confirmación al cliente");
                                }
                            } else {
                                // El cliente ha rechazado todas las sugerencias
                                // Marcar todos los asientos sugeridos como disponibles ('F')
                                for suggestion in seat_suggestions {
                                    for &(section, row, number) in &suggestion {
                                        mark_seat_as('F', seats.clone(), section, row, number);
                                    }
                                }

                                println!("El cliente ha rechazado todas las sugerencias");
                                // Enviar notificación al cliente
                                if ws_sender.send(TungsteniteMessage::Text("Sugerencias rechazadas".to_string())).await.is_err() {
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