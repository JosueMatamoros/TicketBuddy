// src/test.rs
use crate::seat_manager::{mark_seat_as, Seat, Section};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::time::{sleep, Duration};

/// Mark burned seats as booked
pub fn mark_predefined_seats_as_booked(seats: Arc<Mutex<HashMap<(Section, u32, u32), Seat>>>) {
    let predefined_seats = vec![
        ("A1", 1, 4),
        ("A1", 1, 5),
        ("A1", 2, 1),
        ("A1", 2, 2),
        ("A1", 2, 3),
        ("B1", 1, 2),
        ("B1", 1, 3),
        ("B1", 1, 4),
        ("B1", 2, 1),
        ("B1", 2, 5),
        ("C1", 1, 2),
        ("C1", 1, 4),
        ("C1", 2, 1),
        ("C1", 2, 3),
        ("C1", 2, 5),
        ("A2", 1, 1),
        ("A2", 1, 5),
        ("A2", 2, 2),
        ("A2", 2, 4),
        ("A2", 2, 6),
        ("A2", 3, 1),
        ("A2", 3, 3),
        ("A2", 3, 5),
        ("A2", 4, 2),
        ("A2", 4, 4),
        ("B2", 1, 2),
        ("B2", 2, 3),
        ("B2", 3, 4),
        ("B2", 4, 5),
        ("C2", 1, 5),
        ("C2", 2, 4),
        ("C2", 3, 3),
        ("C2", 4, 2),
        ("A3", 2, 1),
        ("A3", 2, 2),
        ("A3", 2, 3),
        ("A3", 2, 4),
        ("A3", 2, 5),
        ("A3", 2, 6),
        ("A3", 4, 1),
        ("A3", 4, 2),
        ("A3", 4, 3),
        ("A3", 4, 4),
        ("A3", 4, 5),
        ("A3", 4, 6),
        ("B3", 1, 1),
        ("B3", 1, 2),
        ("B3", 1, 3),
        ("B3", 1, 4),
        ("B3", 1, 5),
        ("B3", 1, 6),
        ("B3", 2, 3),
        ("B3", 2, 4),
        ("B3", 3, 2),
        ("B3", 3, 3),
        ("B3", 3, 4),
        ("B3", 3, 5),
        ("B3", 4, 1),
        ("B3", 4, 2),
        ("B3", 4, 3),
        ("B3", 4, 4),
        ("B3", 4, 5),
        ("B3", 4, 6),
        ("C3", 1, 1),
        ("C3", 1, 2),
        ("C3", 1, 3),
        ("C3", 1, 4),
        ("C3", 1, 5),
        ("C3", 1, 6),
        ("C3", 3, 1),
        ("C3", 3, 2),
        ("C3", 3, 3),
        ("C3", 3, 4),
        ("C3", 3, 5),
        ("C3", 3, 6),
        ("D", 1, 1),
        ("D", 1, 2),
        ("D", 1, 3),
        ("D", 1, 4),
        ("D", 2, 2),
        ("D", 3, 1),
        ("D", 3, 2),
        ("D", 4, 3),
        ("E", 1, 3),
        ("E", 1, 4),
        ("E", 1, 5),
        ("E", 1, 6),
        ("E", 2, 3),
        ("E", 2, 4),
        ("E", 2, 7),
        ("E", 2, 8)
    ];

    for (section, row, number) in predefined_seats {
        let section_enum = match section {
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
            _ => continue, // Ignore any other section
        };

        // Mark the seat as booked
        mark_seat_as('B', Arc::clone(&seats), section_enum, row, number);
    }
}
