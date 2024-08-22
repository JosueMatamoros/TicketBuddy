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
    let mut best_seats_options = Vec::new();

    // Encuentra la cantidad máxima de filas y asientos por fila para la sección dada
    let max_row = seats
        .keys()
        .filter(|&&(sec, _, _)| sec == section)
        .map(|&(_, row, _)| row)
        .max()
        .unwrap_or(0);

    let max_number = seats
        .keys()
        .filter(|&&(sec, row, _)| sec == section && row == current_row)
        .map(|&(_, _, number)| number)
        .max()
        .unwrap_or(0);

    while available_seats.len() < seats_amount as usize && current_row <= max_row {

        let mut row_seats = Vec::new();

        for number in 1..=max_number {
            if let Some(seat) = seats.get(&(section, current_row, number)) {
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
            return available_seats;
        }else {
            if row_seats.len() > 1 {
                best_seats_options.push(row_seats.clone());
            }
            available_seats.clear();
        }

        current_row += 1;

    }
        if available_seats.len() == 0 {
            find_closest_sets(best_seats_options, seats,section, seats_amount, max_row, max_number)
        }
        else { Vec::new() }
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
        return find_sets_avaible(seats, section, seats_amount, max_row, max_number);
    }

    // Primera fase: buscar en las filas adyacentes
    for option in &seats_options {
        let mut temp_set = Vec::new();

        for &(section, row, number) in option {
            // Agregar el asiento original
            temp_set.push((section, row, number));

            // Buscar en la fila superior
            if row < max_row {
                if let Some(seat) = seats.get(&(section, row + 1, number)) {
                    if seat.booked == 'F' {
                        temp_set.push((section, row + 1, number));
                    }
                }
            }

            // Buscar en la fila inferior
            if row > 1 {
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
        if !temp_set.is_empty() {
            closest_sets.extend(temp_set);
        }

        if closest_sets.len() >= seats_amount as usize {
            closest_sets.truncate(seats_amount as usize);
            return closest_sets;
        }
    }

    // Segunda fase: intentar combinar las mejores opciones
    let mut best_combination = Vec::new();

    for option in seats_options {
        best_combination.extend(option);

        if best_combination.len() >= seats_amount as usize {
            break;
        }
    }

    if best_combination.len() >= seats_amount as usize {
        best_combination.truncate(seats_amount as usize);
        return best_combination;
    }

    // Si no se encontraron suficientes asientos, intentar buscar en toda la sección
    find_sets_avaible(seats, section, seats_amount, max_row, max_number)
}

fn find_sets_avaible(
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

    // Si no se encontraron suficientes asientos, devolver lista vacía
    if available_seats.len() < seats_amount as usize {
        return Vec::new();
    }

    available_seats
}


fn mark_seat_as(state:char, seats: &mut HashMap<(Section, u32, u32), Seat>, section: Section, row: u32, number: u32) {
    if let Some(seat) = seats.get_mut(&(section, row, number)) {
        seat.booked = state;
    }
}

pub fn test (seat: HashMap<(Section, u32, u32), Seat>) {
    let mut seats = seat;
    let section = Section::A1;
    mark_seat_as('B', &mut seats, section, 1, 2);
    mark_seat_as('B', &mut seats, section, 1, 5);
    mark_seat_as('B', &mut seats, section, 2, 3);
    mark_seat_as('B', &mut seats, section, 2, 5);
    mark_seat_as('B', &mut seats, section, 1, 3);
    mark_seat_as('B', &mut seats, section, 2, 1);

    let available_seats = find_seats_by_section(3, section, &seats);
    println!("{:?}", available_seats);
}
