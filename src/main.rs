mod client;
mod seat_manager;
mod socket_manager;
mod test;
use crate::seat_manager::create_seats;
use crate::test::mark_predefined_seats_as_booked;
use socket_manager::start_socket_server;

#[tokio::main]
async fn main() {
    // Create the seats
    let seats = create_seats();

    // Mark the burned seats as booked
    mark_predefined_seats_as_booked(seats.clone());

    // Start the socket server
    start_socket_server(seats).await;
}
