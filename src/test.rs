// src/test.rs
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use tokio::task;
use crate::seat_manager::{find_seats_across_sections, Section, Seat, mark_seat_as};
use tokio::time::{sleep, Duration};

pub async fn run_test(seats: Arc<Mutex<HashMap<(Section, u32, u32), Seat>>>) -> Arc<Mutex<HashMap<(Section, u32, u32), Seat>>> {
    let sections = vec![Section::A1, Section::B1, Section::C1];

    // Marcar un asiento específico como 'R' directamente y verificar


    let mut handles = vec![];

    // Realizar 10 solicitudes concurrentes a `find_seats_across_sections`
    for i in 0..110 {
        let seats_clone = Arc::clone(&seats); // Clonar el Arc para cada tarea
        let section = sections[i % sections.len()]; // Distribuir solicitudes entre A1, B1, C1

        let handle = tokio::spawn(async move {
            let available_seats = find_seats_across_sections(3, section, seats_clone);
            println!("Client {} requested section {:?} and found seats: {:?}", i, section, available_seats);
        });

        handles.push(handle);
        // Pausa de 500 milisegundos entre cada solicitud
        sleep(Duration::from_millis(500)).await;
    }

    // Esperar a que todas las tareas terminen
    for handle in handles {
        handle.await.unwrap();
    }

    seats
}