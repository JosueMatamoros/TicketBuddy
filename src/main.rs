mod seat_manager;

use seat_manager::{Seat, Section, create_seats};
fn main() {
    let seats = create_seats();
    let count = seats.len();


    println!("Total seats: {}", count);
}