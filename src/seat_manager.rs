// Dependencies:
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

pub fn create_seats() -> HashMap<(Section, u32, u32), Seat> {
    let mut seats = HashMap::new();

    let sections_abc1 = vec![Section::A1, Section::B1, Section::C1];
    let sections_abc2 = vec![Section::A2, Section::B2, Section::C2];
    let sections_abc3 = vec![Section::A3, Section::B3, Section::C3];
    let sections_defgh = vec![Section::D, Section::E, Section::F, Section::G, Section::H];

    add_seats(&mut seats, &sections_abc1, 1..=2, 1..=5, 100.0, 150.0);
    add_seats(&mut seats, &sections_abc2, 1..=4, 1..=6, 90.0, 90.0);
    add_seats(&mut seats, &sections_abc3, 1..=4, 1..=6, 80.0, 80.0);
    add_seats(&mut seats, &sections_defgh, 1..=4, 1..=8, 70.0, 30.0);

    seats
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
    seats: &HashMap<(Section, u32, u32), Seat>,
) -> Vec<(Section, u32, u32)> {
    let mut available_seats = Vec::new();
    let mut current_row = 1;

    // Encuentra la cantidad máxima de filas y asientos por fila para la sección dada
    let max_row = seats
        .keys()
        .filter(|&&(sec, _, _)| sec == section)
        .map(|&(_, row, _)| row)
        .max()
        .unwrap_or(0);

    while available_seats.len() < seats_amount as usize && current_row <= max_row {
        let max_number = seats
            .keys()
            .filter(|&&(sec, row, _)| sec == section && row == current_row)
            .map(|&(_, _, number)| number)
            .max()
            .unwrap_or(0);

        let mut row_seats = Vec::new();

        for number in 1..=max_number {
            if let Some(seat) = seats.get(&(section, current_row, number)) {
                if seat.booked == 'F' {
                    row_seats.push((section, current_row, number));
                    if row_seats.len() >= seats_amount as usize {
                        break;
                    }
                } else {
                    row_seats.clear();
                }
            }
        }

        available_seats.extend(row_seats);

        if available_seats.len() >= seats_amount as usize {
            break;
        }

        current_row += 1;
    }

    available_seats.truncate(seats_amount as usize);
    available_seats
}


pub fn test () {
    let available_seats = find_seats_by_section(3, Section::A1, &create_seats());
    println!("Available seats: {:?}", available_seats);
}
