// socket_manager.rs

use crate::seat_manager::{find_seats_suggestions_by_category, get_seat_states, mark_seat_as, Seat, Category, Section};
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
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

#[derive(Serialize, Debug)]
struct SeatSuggestion {
    suggestion_number: usize,
    seats: Vec<SeatInfo>,
    total_price: f32,
}

#[derive(Serialize, Debug)]
struct SeatInfo {
    section: Section,
    row: u32,
    number: u32,
    price: f32,
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
                    // Lista para almacenar los asientos reservados por este cliente
                    let mut client_reserved_seats: Vec<(Section, u32, u32)> = Vec::new();
                    let mut seat_suggestions: Vec<Vec<(Section, u32, u32)>> = Vec::new();

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

                    // Bucle para manejar los mensajes del cliente
                    while let Some(message) = ws_receiver.next().await {
                        match message {
                            Ok(TungsteniteMessage::Text(request_str)) => {
                                println!("Mensaje recibido: {}", request_str);

                                // Intentar parsear como JSON
                                let parsed_message: serde_json::Value = serde_json::from_str(&request_str).unwrap_or(serde_json::Value::Null);

                                if parsed_message["type"] == "payment_result" {
                                    // Manejar resultado del pago
                                    let success = parsed_message["success"].as_bool().unwrap_or(false);
                                    let seats_array_data = parsed_message["seats"].as_array().unwrap_or(&Vec::new()).clone();

                                    let seats_to_update: Vec<(Section, u32, u32)> = seats_array_data.iter().filter_map(|seat_info| {
                                        let section_str = seat_info["section"].as_str().unwrap_or("");
                                        let section = match section_str {
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
                                            _ => return None,
                                        };
                                        let row = seat_info["row"].as_u64().unwrap_or(0) as u32;
                                        let number = seat_info["number"].as_u64().unwrap_or(0) as u32;
                                        Some((section, row, number))
                                    }).collect();

                                    if success {
                                        // Marcar asientos como 'B'
                                        for (section, row, number) in &seats_to_update {
                                            mark_seat_as('B', seats.clone(), *section, *row, *number);
                                            // Remover los asientos de client_reserved_seats
                                            client_reserved_seats.retain(|&seat| seat != (*section, *row, *number));
                                        }
                                        // Enviar estado actualizado de asientos
                                        let seat_states = get_seat_states(seats.clone());
                                        let seat_states_json = serde_json::to_string(&seat_states).unwrap();
                                        if ws_sender.send(TungsteniteMessage::Text(seat_states_json)).await.is_err() {
                                            eprintln!("Error al enviar el estado de los asientos al cliente");
                                        }
                                        // Enviar confirmación
                                        if ws_sender.send(TungsteniteMessage::Text("Pago exitoso".to_string())).await.is_err() {
                                            eprintln!("Error al enviar confirmación al cliente");
                                        }
                                    } else {
                                        // Liberar asientos
                                        for (section, row, number) in &seats_to_update {
                                            mark_seat_as('F', seats.clone(), *section, *row, *number);
                                            client_reserved_seats.retain(|&seat| seat != (*section, *row, *number));
                                        }
                                        // Enviar mensaje de error
                                        if ws_sender.send(TungsteniteMessage::Text("Pago fallido. Intente nuevamente.".to_string())).await.is_err() {
                                            eprintln!("Error al enviar mensaje al cliente");
                                        }
                                    }
                                } else {
                                    // Intentar parsear como SeatRequest
                                    let seat_request: Result<SeatRequest, _> = serde_json::from_str(&request_str);

                                    if let Ok(seat_request) = seat_request {
                                        // Obtener las sugerencias de asientos
                                        seat_suggestions = find_seats_suggestions_by_category(
                                            seat_request.seat_count,
                                            seat_request.category,
                                            seats.clone(),
                                        );

                                        println!("Sugerencias encontradas: {:?}", seat_suggestions);

                                        // Marcar los asientos sugeridos como reservados temporalmente ('R')
                                        for suggestion in &seat_suggestions {
                                            for &(section, row, number) in suggestion {
                                                mark_seat_as('R', seats.clone(), section, row, number);

                                                // Agregar el asiento a la lista de asientos reservados por el cliente
                                                client_reserved_seats.push((section, row, number));
                                            }
                                        }

                                        // Formatear las sugerencias para enviarlas al cliente
                                        let formatted_suggestions: Vec<SeatSuggestion> = {
                                            let seats_guard = seats.lock().unwrap();

                                            seat_suggestions.iter().enumerate().map(|(index, seats_vec)| {
                                                let mut total_price = 0.0;
                                                let seat_infos: Vec<SeatInfo> = seats_vec.iter()
                                                    .map(|(section, row, number)| {
                                                        let seat = seats_guard.get(&(*section, *row, *number)).unwrap();
                                                        total_price += seat.price;
                                                        SeatInfo {
                                                            section: *section,
                                                            row: *row,
                                                            number: *number,
                                                            price: seat.price,
                                                        }
                                                    })
                                                    .collect();

                                                SeatSuggestion {
                                                    suggestion_number: index + 1,
                                                    seats: seat_infos,
                                                    total_price,
                                                }
                                            }).collect()
                                        };

                                        println!("Sugerencias formateadas: {:?}", formatted_suggestions);

                                        // Enviar las sugerencias en formato JSON
                                        let suggestions_json = serde_json::to_string(&formatted_suggestions).unwrap();
                                        if ws_sender.send(TungsteniteMessage::Text(suggestions_json)).await.is_err() {
                                            eprintln!("Error al enviar las sugerencias al cliente");
                                            break; // Salir del bucle si hay un error al enviar
                                        }
                                    } else {
                                        // Manejo de la elección del cliente
                                        let choice: usize = request_str.trim().parse().unwrap_or(0);

                                        if choice >= 1 && choice <= seat_suggestions.len() {
                                            // El cliente ha aceptado una de las sugerencias
                                            let _accepted_seats = &seat_suggestions[choice - 1];

                                            // Los asientos ya están marcados como 'R'

                                            // Marcar las otras sugerencias como disponibles ('F')
                                            for (i, suggestion) in seat_suggestions.iter().enumerate() {
                                                if i != (choice - 1) {
                                                    for &(section, row, number) in suggestion {
                                                        mark_seat_as('F', seats.clone(), section, row, number);
                                                        client_reserved_seats.retain(|&seat| seat != (section, row, number));
                                                    }
                                                }
                                            }

                                            println!("El cliente ha aceptado la sugerencia {}", choice);
                                            // Enviar confirmación al cliente
                                            if ws_sender.send(TungsteniteMessage::Text("Sugerencia aceptada".to_string())).await.is_err() {
                                                eprintln!("Error al enviar la confirmación al cliente");
                                                break; // Salir del bucle si hay un error al enviar
                                            }
                                        } else {
                                            // El cliente ha rechazado todas las sugerencias
                                            // Marcar todos los asientos sugeridos como disponibles ('F')
                                            for suggestion in &seat_suggestions {
                                                for &(section, row, number) in suggestion {
                                                    mark_seat_as('F', seats.clone(), section, row, number);
                                                    client_reserved_seats.retain(|&seat| seat != (section, row, number));
                                                }
                                            }

                                            println!("El cliente ha rechazado todas las sugerencias");
                                            // Enviar notificación al cliente
                                            if ws_sender.send(TungsteniteMessage::Text("Sugerencias rechazadas".to_string())).await.is_err() {
                                                eprintln!("Error al enviar la notificación al cliente");
                                                break;
                                            }
                                        }
                                    }
                                }
                            },
                            Ok(_) => {
                                // Ignorar otros tipos de mensajes (Binary, Ping, Pong, etc.)
                            },
                            Err(e) => {
                                eprintln!("Error en la conexión WebSocket: {}", e);
                                break;
                            }
                        }
                    }

                    // Al finalizar la tarea (cliente desconectado), liberar los asientos reservados por este cliente
                    for &(section, row, number) in &client_reserved_seats {
                        mark_seat_as('F', seats.clone(), section, row, number);
                    }
                    println!("Asientos liberados para el cliente.");

                });
            }
            Err(e) => {
                eprintln!("Error al aceptar conexión: {}", e);
            }
        }
    }
}
