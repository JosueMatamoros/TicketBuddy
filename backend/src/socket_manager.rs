// socket_manager.rs
use crate::seat_manager::{find_seats_across_sections, mark_seat_as, Seat, Section};
use futures_util::{SinkExt, StreamExt};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use tungstenite::protocol::Message as TungsteniteMessage;
use mark_predefined_seats_as_booked;

pub async fn start_socket_server(seats: Arc<Mutex<HashMap<(Section, u32, u32), Seat>>>) {
    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(&addr).await.expect("Failed to bind");

    println!("WebSocket server started at: {}", addr);

    loop {
        match listener.accept().await {
            Ok((stream, _)) => {
                let seats = Arc::clone(&seats);
                tokio::spawn(async move {
                    let ws_stream = accept_async(stream)
                        .await
                        .expect("Error during the websocket handshake occurred");

                    let (mut ws_sender, mut ws_receiver) = ws_stream.split();

                    // Esperar a que el cliente envíe el número de asientos
                    if let Some(Ok(TungsteniteMessage::Text(seat_count_str))) = ws_receiver.next().await {
                        println!("Received seat count: {}", seat_count_str);

                        // Parsear el número de asientos
                        let seat_count: u32 = seat_count_str.trim().parse().unwrap_or(1);

                        // Asignar la sección "A1" por defecto
                        let preferred_section = Section::A1;

                        // Encontrar asientos disponibles
                        let available_seats = find_seats_across_sections(
                            seat_count,
                            preferred_section,
                            seats.clone(),
                        );

                        // Formatear los asientos disponibles para enviarlos al cliente
                        let formatted_seats: Vec<String> = available_seats
                            .iter()
                            .map(|(section, row, number)| {
                                format!("{:?}-Row{}-Seat{}", section, row, number)
                            })
                            .collect();

                        let seat_suggestion_message = TungsteniteMessage::Text(
                            formatted_seats.join(", "),
                        );

                        // Enviar los asientos disponibles al cliente
                        if ws_sender.send(seat_suggestion_message).await.is_err() {
                            eprintln!("Failed to send seat suggestion message");
                        }

                        // Aquí puedes añadir lógica adicional si necesitas que el cliente confirme la reserva
                    }
                });
            }
            Err(e) => {
                eprintln!("Failed to accept connection: {}", e);
            }
        }
    }
}
