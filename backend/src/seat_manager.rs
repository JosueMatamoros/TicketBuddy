use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use serde::{Serialize, Deserialize};

/// Estructura que representa un asiento.
#[derive(Debug)]
pub struct Seat {
    pub number: u32,
    pub section: Section,
    pub row: u32,
    pub visibility: f32,
    pub price: f32,
    pub booked: char, // 'B' = Reservado, 'R' = Reservado temporalmente, 'F' = Libre
}

#[derive(Debug, Serialize)]
pub struct SeatState {
    pub section: Section,
    pub row: u32,
    pub number: u32,
    pub booked: char,
}

/// Enumeración que representa las diferentes secciones en la disposición de asientos.
#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy, Serialize, Deserialize)]
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
}

/// Implementación de métodos para la enumeración Section.
impl Section {
    /// Función para obtener todas las secciones.
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
        ]
    }
}

/// Función para crear el conjunto de asientos.
/// Retorna un `Arc<Mutex<HashMap<...>>>` que contiene todos los asientos.
pub fn create_seats() -> Arc<Mutex<HashMap<(Section, u32, u32), Seat>>> {
    let mut seats = HashMap::new();

    let sections_abc1 = vec![Section::A1, Section::B1, Section::C1];
    let sections_abc2 = vec![Section::A2, Section::B2, Section::C2];
    let sections_abc3 = vec![Section::A3, Section::B3, Section::C3];
    let sections_defgh = vec![Section::D, Section::E, Section::F];

    add_seats(&mut seats, &sections_abc1, 1..=2, 1..=5, 100.0, 150.0);
    add_seats(&mut seats, &sections_abc2, 1..=4, 1..=6, 90.0, 90.0);
    add_seats(&mut seats, &sections_abc3, 1..=4, 1..=6, 80.0, 80.0);
    add_seats(&mut seats, &sections_defgh, 1..=4, 1..=8, 70.0, 30.0);

    Arc::new(Mutex::new(seats))
}

/// Función auxiliar para añadir asientos a la disposición.
/// Modifica el `HashMap` proporcionado con las secciones, filas y números especificados.
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

/// Función para encontrar las tres secciones con más asientos disponibles.
pub fn find_top_three_sections(
    seats: Arc<Mutex<HashMap<(Section, u32, u32), Seat>>>,
) -> Vec<Section> {
    let seats_guard = seats.lock().unwrap();

    // Crear un HashMap para contar asientos disponibles por sección
    let mut section_counts: HashMap<Section, u32> = HashMap::new();

    for ((section, _, _), seat) in seats_guard.iter() {
        if seat.booked == 'F' {
            *section_counts.entry(*section).or_insert(0) += 1;
        }
    }

    // Convertir los conteos en un Vec y ordenar por conteos descendentes
    let mut section_counts_vec: Vec<(Section, u32)> = section_counts.into_iter().collect();
    section_counts_vec.sort_by(|a, b| b.1.cmp(&a.1));

    // Tomar las tres primeras secciones
    let top_three_sections: Vec<Section> = section_counts_vec
        .into_iter()
        .take(3)
        .map(|(section, _)| section)
        .collect();

    top_three_sections
}

/// Nueva función para encontrar sugerencias de asientos en las tres secciones con más asientos disponibles.
pub fn find_seats_suggestions(
    seats_amount: u32,
    seats: Arc<Mutex<HashMap<(Section, u32, u32), Seat>>>,
) -> Vec<Vec<(Section, u32, u32)>> {
    let top_sections = find_top_three_sections(Arc::clone(&seats));
    let mut suggestions = Vec::new();
    let mut visited_sections = Vec::new();

    for &section in &top_sections {
        let available_seats = find_seats_by_section(
            seats_amount,
            section,
            Arc::clone(&seats),
            &mut visited_sections,
        );

        if !available_seats.is_empty() {
            suggestions.push(available_seats);
        }
    }

    suggestions
}

/// Función para encontrar asientos disponibles en una sección específica.
fn find_seats_by_section(
    seats_amount: u32,
    section: Section,
    seats: Arc<Mutex<HashMap<(Section, u32, u32), Seat>>>,
    visited_sections: &mut Vec<Section>,
) -> Vec<(Section, u32, u32)> {
    // Evitar visitar la misma sección varias veces
    if !visited_sections.contains(&section) {
        visited_sections.push(section);
    }

    let seats_guard = seats.lock().unwrap(); // Bloquear el Mutex
    let mut available_seats = Vec::new();
    let mut current_row = 1;
    let mut best_seats_options = Vec::new();

    // Encontrar la fila máxima en la sección
    let max_row = seats_guard
        .keys()
        .filter(|&&(sec, _, _)| sec == section)
        .map(|&(_, row, _)| row)
        .max()
        .unwrap_or(0);

    // Encontrar el número máximo en la sección
    let max_number = seats_guard
        .keys()
        .filter(|&&(sec, row, _)| sec == section && row == current_row)
        .map(|&(_, _, number)| number)
        .max()
        .unwrap_or(0);

    // Buscar asientos disponibles en la sección
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

        // Añadir los asientos encontrados a la lista de asientos disponibles
        available_seats.extend(row_seats.clone());

        // Si se han encontrado suficientes asientos, romper el bucle
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

    // Si no se encontraron suficientes asientos, buscar en filas adyacentes
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

    // Truncar la lista de asientos disponibles al número deseado
    available_seats.truncate(seats_amount as usize);
    available_seats
}

/// Función auxiliar para encontrar los conjuntos más cercanos de asientos disponibles.
fn find_closest_sets(
    seats_options: Vec<Vec<(Section, u32, u32)>>,
    seats: &HashMap<(Section, u32, u32), Seat>,
    section: Section,
    seats_amount: u32,
    max_row: u32,
    max_number: u32,
) -> Vec<(Section, u32, u32)> {
    let mut closest_sets = Vec::new();

    // Si no hay opciones, intentar encontrar asientos disponibles en la sección
    if seats_options.is_empty() {
        return find_sets_available(seats, section, seats_amount, max_row, max_number);
    }

    // Buscar en filas adyacentes y combinar opciones si es necesario
    for option in &seats_options {
        let mut temp_set = Vec::new();

        for &(section, row, number) in option {
            temp_set.push((section, row, number));

            if temp_set.len() >= seats_amount as usize {
                return temp_set;
            }

            // Buscar asientos en las filas adyacentes
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
        // Añadir los asientos encontrados al conjunto más cercano
        closest_sets.extend(temp_set);

        if closest_sets.len() >= seats_amount as usize {
            closest_sets.truncate(seats_amount as usize);
            return closest_sets;
        }
    }

    // Si aún no se han encontrado suficientes asientos, buscar en toda la sección
    let mut best_combination = Vec::new();

    // Combinar todas las opciones
    for option in seats_options {
        best_combination.extend(option);

        if best_combination.len() >= seats_amount as usize {
            best_combination.truncate(seats_amount as usize);
            return best_combination;
        }
    }

    // Si aún no se han encontrado suficientes asientos, buscar en toda la sección
    find_sets_available(seats, section, seats_amount, max_row, max_number)
}

/// Función para encontrar asientos disponibles en toda la disposición.
fn find_sets_available(
    seats: &HashMap<(Section, u32, u32), Seat>,
    section: Section,
    seats_amount: u32,
    max_row: u32,
    max_number: u32,
) -> Vec<(Section, u32, u32)> {
    let mut available_seats = Vec::new();

    // Buscar asientos disponibles en toda la sección
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

/// Función para encontrar los primeros `n` asientos disponibles en toda la disposición.
pub fn find_first_n_available_seats(
    seats_amount: u32,
    seats: Arc<Mutex<HashMap<(Section, u32, u32), Seat>>>,
) -> Vec<(Section, u32, u32)> {
    let seats_guard = seats.lock().unwrap(); // Bloquear el Mutex
    let mut available_seats = Vec::new();

    // Buscar los primeros `n` asientos disponibles
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

/// Función para marcar un asiento con un estado específico.
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

/// Función para encontrar asientos disponibles en toda la disposición.
// Esta función no es necesaria para la funcionalidad principal.
pub fn find_available_seats(
    seats: Arc<Mutex<HashMap<(Section, u32, u32), Seat>>>,
) -> Vec<(Section, u32, u32)> {
    let seats_guard = seats.lock().unwrap(); // Bloquear el Mutex
    let mut available_seats = Vec::new();

    for (&key, seat) in seats_guard.iter() {
        if seat.booked == 'F' {
            println!("Asiento disponible: {:?}", key);
            available_seats.push(key);
        }
    }

    available_seats
}

/// Función para obtener el estado actual de todos los asientos
pub fn get_seat_states(
    seats: Arc<Mutex<HashMap<(Section, u32, u32), Seat>>>,
) -> Vec<SeatState> {
    let seats_guard = seats.lock().unwrap();
    let mut seat_states = Vec::new();

    for ((section, row, number), seat) in seats_guard.iter() {
        seat_states.push(SeatState {
            section: *section,
            row: *row,
            number: *number,
            booked: seat.booked,
        });
    }

    seat_states
}
