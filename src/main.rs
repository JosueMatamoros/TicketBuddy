mod client;
mod seat_manager;
mod socket_manager;

use socket_manager::start_socket_server;
use tokio::runtime::Runtime;
use crate::seat_manager::create_seats;

fn main() {

    // let rt = Runtime::new().unwrap();
    // rt.block_on(async {
    //     start_socket_server().await;
    // });
    seat_manager::test(create_seats());
}
