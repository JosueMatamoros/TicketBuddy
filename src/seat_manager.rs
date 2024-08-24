use std::sync::{Arc, Mutex};
use std::collections::HashMap;

/// Structure representing a seat.
#[derive(Debug)]
pub struct Seat {
    pub number: u32,
    pub section: Section,
    pub row: u32,
    pub visibility: f32,
    pub price: f32,
    pub booked: char, // B = Booked, R = Reserved, F = Free
}

/// Enum representing different sections in the seating arrangement.
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

/// Function to create a set of seats.
/// Returns an `Arc<Mutex<HashMap<...>>>` containing all the seats.
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

/// Helper function to add seats to the seating arrangement.
/// Modifies the provided `HashMap` with the specified sections, rows, and numbers.
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

/// Function to find available seats in a specific section.
/// If enough seats are not found, it attempts to find seats in other sections,
/// and if that fails, it searches across the entire seating arrangement.
/// The seats are marked as reserved ('R') once found.
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

    let mut result = if available_seats.is_empty() {
        find_closest_sets(best_seats_options, &seats_guard, section, seats_amount, max_row, max_number)
    } else {
        println!("Encontrados a la primer yupiii");
        available_seats
    };

    // Si no se encontró la cantidad exacta, buscar en otras secciones
    if result.len() < seats_amount as usize {
        let mut visited_sections = vec![section];
        result = find_in_other_sections(seats_amount, section, &seats_guard, &mut visited_sections);
    }

    // Marcar los asientos encontrados como reservados (R)
    for &(section, row, number) in &result {
        if let Some(seat) = seats_guard.get_mut(&(section, row, number)) {
            seat.booked = 'R';
        }
    }

    result
}

/// Helper function to find the closest sets of available seats.
/// This function is called when not enough seats are found in the direct search.
fn find_closest_sets(
    seats_options: Vec<Vec<(Section, u32, u32)>>,
    seats: &HashMap<(Section, u32, u32), Seat>,
    section: Section,
    seats_amount: u32,
    max_row: u32,
    max_number: u32,
) -> Vec<(Section, u32, u32)> {
    let mut closest_sets = Vec::new();
    println!("Estamos buscando asientos cercanos");
    println!("Estmos en la seccion {:?}", section);
    println!("Cantidad de setsOptions: {}", seats_options.len());

    // If no options, attempt to find available seats in the section
    {
    if seats_options.is_empty() {
        return find_sets_available(seats, section, seats_amount, max_row, max_number);
    }
}


    // Search in adjacent rows and combine options if necessary
    for option in &seats_options {
        let mut temp_set = Vec::new();

        for &(section, row, number) in option {
            temp_set.push((section, row, number));

            if temp_set.len() >= seats_amount as usize {
                return temp_set;
            }

            // Look for seats in the adjacent rows
            if row < max_row && temp_set.len() < seats_amount as usize {
                if let Some(seat) = seats.get(&(section, row + 1, number)) {
                    if seat.booked == 'F' {
                        temp_set.push((section, row + 1, number));
                    }
                }
            }

            if row > 1 && temp_set.len() < seats_amount as usize {
                if let Some(seat) = seats.get(&(section, row - 1, number)) {
                    if seat.booked == 'F' {
                        temp_set.push((section, row - 1, number));
                    }
                }
            }

            if temp_set.len() >= seats_amount as usize {
                return temp_set;
            }
        }

        closest_sets.extend(temp_set);

        if closest_sets.len() >= seats_amount as usize {
            closest_sets.truncate(seats_amount as usize);
            return closest_sets;
        }
    }

    // Combine the best options if not enough seats were found
    let mut best_combination = Vec::new();

    for option in seats_options {
        best_combination.extend(option);

        if best_combination.len() >= seats_amount as usize {
            best_combination.truncate(seats_amount as usize);
            return best_combination;
        }
    }

    // If still not enough seats, search the entire section
    find_sets_available(seats, section, seats_amount, max_row, max_number)
}

/// Function to find available seats across the entire seating arrangement.
/// This is called when not enough seats are found in any specific section.
fn find_sets_available(
    seats: &HashMap<(Section, u32, u32), Seat>,
    section: Section,
    seats_amount: u32,
    max_row: u32,
    max_number: u32,
) -> Vec<(Section, u32, u32)> {
    println!("Estamos buscando en toda la disposición de asientos en la sección {:?}", section);
    let mut available_seats = Vec::new();

    for row in 1..=max_row {
        for number in 1..=max_number {
            if let Some(seat) = seats.get(&(section, row, number)) {
                if seat.booked == 'F' {
                    available_seats.push((section, row, number));

                    if available_seats.len() >= seats_amount as usize {
                        return available_seats;
                    }
                }
            }
        }
    }

    available_seats
}

fn find_in_other_sections(
    seats_amount: u32,
    original_section: Section,
    seats: &HashMap<(Section, u32, u32), Seat>,
    visited_sections: &mut Vec<Section>
) -> Vec<(Section, u32, u32)> {
    let adjacent_sections = match original_section {
        Section::A1 | Section::B1 | Section::C1 => vec![Section::A1, Section::B1, Section::C1],
        Section::A2 | Section::B2 | Section::C2 => vec![Section::A2, Section::B2, Section::C2],
        Section::A3 | Section::B3 | Section::C3 => vec![Section::A3, Section::B3, Section::C3],
        _ => vec![Section::D, Section::E, Section::F, Section::G, Section::H],
    };

    let mut available_seats = Vec::new();

    for &section in &adjacent_sections {
        if !visited_sections.contains(&section) {
            visited_sections.push(section);
            let max_row = seats.keys().filter(|&&(sec, _, _)| sec == section).map(|&(_, row, _)| row).max().unwrap_or(0);
            let max_number = seats.keys().filter(|&&(sec, row, _)| sec == section).map(|&(_, _, number)| number).max().unwrap_or(0);
            let seats_in_section = find_sets_available(seats, section, seats_amount, max_row, max_number);
            available_seats.extend(seats_in_section);

            if available_seats.len() >= seats_amount as usize {
                return available_seats;
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
    let mut seats_guard = seats.lock().unwrap();
    if let Some(seat) = seats_guard.get_mut(&(section, row, number)) {
        seat.booked = state;
    }
}
