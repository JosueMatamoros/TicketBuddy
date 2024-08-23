mod client;
mod seat_manager;
mod socket_manager;
use socket_manager::start_socket_server;
use crate::seat_manager::create_seats;
use std::sync::{Arc, Mutex};

// Función principal del servidor
#[tokio::main]
async fn main() {
    // Creación de los asientos y almacenamiento en un Arc<Mutex<...>>
    let seats = Arc::new(Mutex::new(create_seats()));

    start_socket_server(seats).await;
}
