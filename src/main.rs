mod seat_manager;
mod socket_manager;
mod client;

use tokio::runtime::Runtime;
use socket_manager::start_socket_server;

fn main() {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        start_socket_server().await;

    });

}