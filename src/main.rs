mod client;
mod seat_manager;
mod socket_manager;
mod test;


use socket_manager::start_socket_server;
use crate::seat_manager::{create_seats, mark_predefined_seats_as_booked};

// Función principal del servidor
#[tokio::main]
async fn main() {
    // Crear los asientos
    let seats = create_seats();

    // Asientos predefinidos para pruebas
    let predefined_seats = vec![
        ("A1", 1, 4), ("A1", 1, 5), ("A1", 2, 1), ("A1", 2, 2), ("A1", 2, 3),
        ("B1", 1, 2), ("B1", 1, 3), ("B1", 1, 4), ("B1", 2, 1), ("B1", 2, 5),
        ("C1", 1, 2), ("C1", 1, 4), ("C1", 2, 1), ("C1", 2, 3), ("C1", 2, 5),
        ("A2", 1, 1), ("A2", 1, 5), ("A2", 2, 2), ("A2", 2, 4), ("A2", 2, 6),
        ("A2", 3, 1), ("A2", 3, 3), ("A2", 3, 5), ("A2", 4, 2), ("A2", 4, 4),
        ("B2", 1, 2), ("B2", 2, 3), ("B2", 3, 4), ("B2", 4, 5),
        ("C2", 1, 5), ("C2", 2, 4), ("C2", 3, 3), ("C2", 4, 2),
        ("A3", 2, 1), ("A3", 2, 2), ("A3", 2, 3), ("A3", 2, 4), ("A3", 2, 5), ("A3", 2, 6),
        ("A3", 4, 1), ("A3", 4, 2), ("A3", 4, 3), ("A3", 4, 4), ("A3", 4, 5), ("A3", 4, 6),
        ("B3", 1, 1), ("B3", 1, 2), ("B3", 1, 3), ("B3", 1, 4), ("B3", 1, 5), ("B3", 1, 6),
        ("B3", 2, 3), ("B3", 2, 4), ("B3", 3, 2), ("B3", 3, 3), ("B3", 3, 4), ("B3", 3, 5),
        ("B3", 4, 1), ("B3", 4, 2), ("B3", 4, 3), ("B3", 4, 4), ("B3", 4, 5), ("B3", 4, 6),
        ("C3", 1, 1), ("C3", 1, 2), ("C3", 1, 3), ("C3", 1, 4), ("C3", 1, 5), ("C3", 1, 6),
        ("C3", 3, 1), ("C3", 3, 2), ("C3", 3, 3), ("C3", 3, 4), ("C3", 3, 5), ("C3", 3, 6),
        ("D", 1, 1), ("D", 1, 2), ("D", 1, 3), ("D", 1, 4), ("D", 2, 2), ("D", 3, 1), ("D", 3, 2), ("D", 4, 3),
        ("E", 1, 3), ("E", 1, 4), ("E", 1, 5), ("E", 1, 6), ("E", 2, 3), ("E", 2, 4), ("E", 2, 7), ("E", 2, 8),
        ("F", 2, 2), ("F", 2, 3), ("F", 2, 4), ("F", 4, 5), ("F", 4, 6), ("F", 4, 7), ("F", 4, 8),
        ("G", 1, 1), ("G", 1, 2), ("G", 1, 3), ("G", 1, 4), ("G", 1, 5), ("G", 1, 6), ("G", 1, 7), ("G", 1, 8),
        ("G", 2, 1), ("G", 2, 2), ("G", 2, 3), ("G", 2, 4), ("G", 2, 5), ("G", 2, 6), ("G", 2, 7), ("G", 2, 8),
    ];

    // Ejecuta la función para marcar los asientos predefinidos como reservados
    mark_predefined_seats_as_booked(seats.clone(), predefined_seats);

    // Pasar `seats` correctamente a la función `start_socket_server`
    start_socket_server(seats).await;
}
