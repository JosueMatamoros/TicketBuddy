mod client;
mod seat_manager;
mod socket_manager;
mod test;


use socket_manager::start_socket_server;
use crate::seat_manager::{create_seats, mark_seat_as, Section};
use std::sync::{Arc, Mutex};

// Función principal del servidor
#[tokio::main]
async fn main() {
    let seats = create_seats();
    // Pasar `seats` correctamente a la función `start_socket_server`
    start_socket_server(seats).await;
    // Llamar a la función `run_test` que está en `test.rs`
    // test::run_test(seats).await;
}


