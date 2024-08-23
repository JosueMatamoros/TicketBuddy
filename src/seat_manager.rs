use std::sync::{Arc, Mutex};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Seat {
    pub number: u32,
    pub section: Section,
    pub row: u32,
    pub visibility: f32,
    pub price: f32,
    pub booked: char, // B = Booked, R = Reserved, F = Free
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub enum Section {
    A1,
    B1,
    C1,
    A2,
    B2,
    C2,
    A3,
    B3,
    C3,
    D,
    E,
    F,
    G,
    H,
}

pub fn create_seats() -> Arc<Mutex<HashMap<(Section, u32, u32), Seat>>> {
    let mut seats = HashMap::new();

    let sections_abc1 = vec![Section::A1, Section::B1, Section::C1];
    let sections_abc2 = vec![Section::A2, Section::B2, Section::C2];
    let sections_abc3 = vec![Section::A3, Section::B3, Section::C3];
    let sections_defgh = vec![Section::D, Section::E, Section::F, Section::G, Section::H];

    add_seats(&mut seats, &sections_abc1, 1..=2, 1..=5, 100.0, 150.0);
    add_seats(&mut seats, &sections_abc2, 1..=4, 1..=6, 90.0, 90.0);
    add_seats(&mut seats, &sections_abc3, 1..=4, 1..=6, 80.0, 80.0);
    add_seats(&mut seats, &sections_defgh, 1..=4, 1..=8, 70.0, 30.0);

    Arc::new(Mutex::new(seats))
}

fn add_seats(
    seats: &mut HashMap<(Section, u32, u32), Seat>,
    sections: &[Section],
    row_range: std::ops::RangeInclusive<u32>,
    number_range: std::ops::RangeInclusive<u32>,
    visibility: f32,
    price: f32,
) {
    for &section in sections {
        for row in row_range.clone() {
            for number in number_range.clone() {
                seats.insert(
                    (section, row, number),
                    Seat {
                        number,
                        section,
                        row,
                        visibility,
                        price,
                        booked: 'F',
                    },
                );
            }
        }
    }
}

pub fn find_seats_by_section(
    seats_amount: u32,
    section: Section,
    seats: Arc<Mutex<HashMap<(Section, u32, u32), Seat>>>,
) -> Vec<(Section, u32, u32)> {
    let mut seats_guard = seats.lock().unwrap(); // Bloqueo del Mutex
    let mut available_seats = Vec::new();
    let mut current_row = 1;
    let mut best_seats_options = Vec::new();

    // Encuentra la cantidad máxima de filas y asientos por fila para la sección dada
    let max_row = seats_guard
        .keys()
        .filter(|&&(sec, _, _)| sec == section)
        .map(|&(_, row, _)| row)
        .max()
        .unwrap_or(0);

    let max_number = seats_guard
        .keys()
        .filter(|&&(sec, row, _)| sec == section && row == current_row)
        .map(|&(_, _, number)| number)
        .max()
        .unwrap_or(0);

    while available_seats.len() < seats_amount as usize && current_row <= max_row {
        let mut row_seats = Vec::new();

        for number in 1..=max_number {
            if let Some(seat) = seats_guard.get(&(section, current_row, number)) {
                if seat.booked == 'F' {
                    row_seats.push((section, current_row, number));
                    if row_seats.len() >= seats_amount as usize {
                        break;
                    }
                } else {
                    if row_seats.len() > 1 {
                        best_seats_options.push(row_seats.clone());
                    }
                    row_seats.clear();
                }
            }
        }

        available_seats.extend(row_seats.clone());

        if available_seats.len() >= seats_amount as usize {
            break;
        } else {
            if row_seats.len() > 1 {
                best_seats_options.push(row_seats.clone());
            }
            available_seats.clear();
        }

        current_row += 1;
    }

    let result = if available_seats.is_empty() {
        find_closest_sets(best_seats_options, &seats_guard, section, seats_amount, max_row, max_number)
    } else {
        available_seats
    };

    // Marcar los asientos encontrados como reservados (R)
    for &(section, row, number) in &result {
        if let Some(seat) = seats_guard.get_mut(&(section, row, number)) {
            seat.booked = 'R';
        }
    }

    result
}

fn find_closest_sets(
    seats_options: Vec<Vec<(Section, u32, u32)>>,
    seats: &HashMap<(Section, u32, u32), Seat>,
    section: Section,
    seats_amount: u32,
    max_row: u32,
    max_number: u32,
) -> Vec<(Section, u32, u32)> {
    let mut closest_sets = Vec::new();

    // Verificar si hay opciones de asientos juntos
    if seats_options.is_empty() {
        // Si no hay asientos juntos, intentar buscar asientos disponibles en la sección
        return find_sets_available(seats, section, seats_amount, max_row, max_number);
    }

    // Primera fase: buscar en las filas adyacentes y combinar opciones si es necesario
    for option in &seats_options {
        let mut temp_set = Vec::new();

        for &(section, row, number) in option {
            // Agregar el asiento original
            temp_set.push((section, row, number));

            // Si ya encontramos suficientes asientos, salimos
            if temp_set.len() >= seats_amount as usize {
                return temp_set;
            }

            // Buscar en la fila superior
            if row < max_row && temp_set.len() < seats_amount as usize {
                if let Some(seat) = seats.get(&(section, row + 1, number)) {
                    if seat.booked == 'F' {
                        temp_set.push((section, row + 1, number));
                    }
                }
            }

            // Buscar en la fila inferior
            if row > 1 && temp_set.len() < seats_amount as usize {
                if let Some(seat) = seats.get(&(section, row - 1, number)) {
                    if seat.booked == 'F' {
                        temp_set.push((section, row - 1, number));
                    }
                }
            }

            // Si ya encontramos suficientes asientos, salimos
            if temp_set.len() >= seats_amount as usize {
                return temp_set;
            }
        }

        // Agregar la combinación temporal al conjunto de resultados
        closest_sets.extend(temp_set);

        if closest_sets.len() >= seats_amount as usize {
            closest_sets.truncate(seats_amount as usize);
            return closest_sets;
        }
    }

    // Segunda fase: intentar combinar las mejores opciones si aún no hemos encontrado suficientes asientos
    let mut best_combination = Vec::new();

    for option in seats_options {
        best_combination.extend(option);

        if best_combination.len() >= seats_amount as usize {
            best_combination.truncate(seats_amount as usize);
            return best_combination;
        }
    }

    // Si no se encontraron suficientes asientos, intentar buscar en toda la sección
    find_sets_available(seats, section, seats_amount, max_row, max_number)
}


fn find_sets_available(
    seats: &HashMap<(Section, u32, u32), Seat>,
    section: Section,
    seats_amount: u32,
    max_row: u32,
    max_number: u32,
) -> Vec<(Section, u32, u32)> {
    let mut available_seats = Vec::new();
    for row in 1..=max_row {
        for number in 1..=max_number {
            if let Some(seat) = seats.get(&(section, row, number)) {
                if seat.booked == 'F' {
                    available_seats.push((section, row, number));

                    // Si ya tenemos suficientes asientos, retornarlos
                    if available_seats.len() >= seats_amount as usize {
                        return available_seats;
                    }
                }
            }
        }
    }

    available_seats
}

pub fn mark_seat_as(
    state: char,
    seats: Arc<Mutex<HashMap<(Section, u32, u32), Seat>>>,
    section: Section,
    row: u32,
    number: u32,
) {
    let mut seats = seats.lock().unwrap();
    if let Some(seat) = seats.get_mut(&(section, row, number)) {
        seat.booked = state;
    }
}


