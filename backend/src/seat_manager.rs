use std::collections::HashMap;
use std::sync::{Arc, Mutex};

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

/// Function to get all sections.
impl Section {
    pub fn all_sections() -> Vec<Section> {
        vec![
            Section::A1,
            Section::B1,
            Section::C1,
            Section::A2,
            Section::B2,
            Section::C2,
            Section::A3,
            Section::B3,
            Section::C3,
            Section::D,
            Section::E,
            Section::F,
            Section::G,
            Section::H,
        ]
    }
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

/// Function to find available seats across multiple sections.
/// This is the main entry point for seat finding.
pub fn find_seats_across_sections(
    seats_amount: u32,
    preferred_section: Section,
    seats: Arc<Mutex<HashMap<(Section, u32, u32), Seat>>>,
) -> Vec<(Section, u32, u32)> {
    let mut visited_sections = Vec::new();
    let mut available_seats = find_seats_by_section(
        seats_amount,
        preferred_section,
        Arc::clone(&seats),
        &mut visited_sections,
    );

    if available_seats.len() < seats_amount as usize {
        let adjacent_sections = Section::all_sections();

        // Search in adjacent sections if not enough seats are found in the preferred section
        for &section in &adjacent_sections {
            if !visited_sections.contains(&section) {
                let seats_in_section = find_seats_by_section(
                    seats_amount,
                    section,
                    Arc::clone(&seats),
                    &mut visited_sections,
                );

                if seats_in_section.len() == seats_amount as usize {
                    available_seats = seats_in_section;
                    break;
                }
            }
        }

        // If still not enough seats, find the first `n` available seats
        if available_seats.len() < seats_amount as usize {
            available_seats = find_first_n_available_seats(seats_amount, Arc::clone(&seats));
        }
    }

    // Mark the found seats as reserved
    for &(section, row, number) in &available_seats {
        mark_seat_as('R', Arc::clone(&seats), section, row, number);
    }

    available_seats
}

/// Function to find available seats in a specific section.
fn find_seats_by_section(
    seats_amount: u32,
    section: Section,
    seats: Arc<Mutex<HashMap<(Section, u32, u32), Seat>>>,
    visited_sections: &mut Vec<Section>,
) -> Vec<(Section, u32, u32)> {
    // Avoid visiting the same section multiple times
    if !visited_sections.contains(&section) {
        visited_sections.push(section);
    }

    let seats_guard = seats.lock().unwrap(); // Mutex lock
    let mut available_seats = Vec::new();
    let mut current_row = 1;
    let mut best_seats_options = Vec::new();

    // Find the maximum row and number in the section
    let max_row = seats_guard
        .keys()
        .filter(|&&(sec, _, _)| sec == section)
        .map(|&(_, row, _)| row)
        .max()
        .unwrap_or(0);

    // Find the maximum number in the section
    let max_number = seats_guard
        .keys()
        .filter(|&&(sec, row, _)| sec == section && row == current_row)
        .map(|&(_, _, number)| number)
        .max()
        .unwrap_or(0);

    // Search for available seats in the section
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

        // Add the found seats to the available seats list
        available_seats.extend(row_seats.clone());

        // If enough seats are found, break the loop
        if available_seats.len() >= seats_amount as usize {
            break;
        } else { // Otherwise, clear the row seats and continue searching
            if row_seats.len() > 1 {
                best_seats_options.push(row_seats.clone());
            }
            available_seats.clear();
        }

        current_row += 1;
    }

    // If not enough seats are found, search in adjacent rows
    if available_seats.len() < seats_amount as usize {
        let closest_seats = find_closest_sets(
            best_seats_options,
            &seats_guard,
            section,
            seats_amount - available_seats.len() as u32,
            max_row,
            max_number,
        );
        available_seats.extend(closest_seats);
    }
    // Truncate the available seats list to the desired amount
    available_seats.truncate(seats_amount as usize);
    available_seats
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

    // If no options, attempt to find available seats in the section
    if seats_options.is_empty() {
        return find_sets_available(seats, section, seats_amount, max_row, max_number);
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

            // Look for seats in the adjacent rows
            if row > 1 && temp_set.len() < seats_amount as usize {
                if let Some(seat) = seats.get(&(section, row - 1, number)) {
                    if seat.booked == 'F' {
                        temp_set.push((section, row - 1, number));
                    }
                }
            }
            // If enough seats are found, return the set
            if temp_set.len() >= seats_amount as usize {
                return temp_set;
            }
        }
        // Add the found seats to the closest sets list
        closest_sets.extend(temp_set);

        // If enough seats are found, truncate the list and return
        if closest_sets.len() >= seats_amount as usize {
            closest_sets.truncate(seats_amount as usize);
            return closest_sets;
        }
    }

    // If still not enough seats, search the entire section
    let mut best_combination = Vec::new();

    // Combine all options
    for option in seats_options {
        best_combination.extend(option);

        // If enough seats are found, truncate the list and return
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
    let mut available_seats = Vec::new();

    // Search for available seats in the entire section
    for row in 1..=max_row {
        for number in 1..=max_number {
            if let Some(seat) = seats.get(&(section, row, number)) {
                if seat.booked == 'F' {
                    available_seats.push((section, row, number));

                    // If enough seats are found, return the list
                    if available_seats.len() >= seats_amount as usize {
                        return available_seats;
                    }
                }
            }
        }
    }

    available_seats
}

/// Function to find the first `n` available seats in the entire seating arrangement.
pub fn find_first_n_available_seats(
    seats_amount: u32,
    seats: Arc<Mutex<HashMap<(Section, u32, u32), Seat>>>,
) -> Vec<(Section, u32, u32)> {
    let seats_guard = seats.lock().unwrap(); // Lock the Mutex
    let mut available_seats = Vec::new();

    // Search for the first `n` available seats
    for (&key, seat) in seats_guard.iter() {
        if seat.booked == 'F' {
            available_seats.push(key);
            if available_seats.len() == seats_amount as usize {
                break;
            }
        }
    }

    available_seats
}

/// Function to mark a seat as reserved or in any other state.
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

/// Function to find available seats in the entire seating arrangement.
// This function is not necessary for the main functionality.
pub fn find_available_seats(
    seats: Arc<Mutex<HashMap<(Section, u32, u32), Seat>>>,
) -> Vec<(Section, u32, u32)> {
    let seats_guard = seats.lock().unwrap(); // Lock the Mutex
    let mut available_seats = Vec::new();

    for (&key, seat) in seats_guard.iter() {
        if seat.booked == 'F' {
            println!("Available seat: {:?}", key);
            available_seats.push(key);
        }
    }

    available_seats
}
