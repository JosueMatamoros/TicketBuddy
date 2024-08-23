// src/test.rs
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use tokio::task;
use crate::seat_manager::{find_seats_by_section, Section, Seat, mark_seat_as};

pub async fn run_test(seats: Arc<Mutex<HashMap<(Section, u32, u32), Seat>>>) {
    let sections = vec![Section::A1, Section::B1, Section::C1];

    // Marcar un asiento específico como 'R' directamente y verificar
    mark_seat_as('R', seats.clone(), sections[0], 1, 2);
    mark_seat_as('R', seats.clone(), sections[1], 1, 1);
    mark_seat_as('R', seats.clone(), sections[1], 1, 5);
    mark_seat_as('R', seats.clone(), sections[1], 2, 2);
     mark_seat_as('R', seats.clone(), sections[1], 2, 3);
     mark_seat_as('R', seats.clone(), sections[1], 2, 4);


    let mut handles = vec![];

    // Realizar 10 solicitudes concurrentes a `find_seats_by_section`
    for i in 0..10 {
        let seats_clone = Arc::clone(&seats); // Clonar el Arc para cada tarea
        let section = sections[i % sections.len()]; // Distribuir solicitudes entre A1, B1, C1

        let handle = tokio::spawn(async move {
            let available_seats = find_seats_by_section(3, section, seats_clone);
            println!("Client {} requested section {:?} and found seats: {:?}", i, section, available_seats);
        });

        handles.push(handle);
    }

    // Esperar a que todas las tareas terminen
    for handle in handles {
        handle.await.unwrap();
    }
}
