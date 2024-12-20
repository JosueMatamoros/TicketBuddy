// seat_manager.rs

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

/// Estructura para serializar el estado del asiento
#[derive(Debug, Serialize)]
pub struct SeatState {
    pub section: Section,
    pub row: u32,
    pub number: u32,
    pub booked: char,
}

/// Enumeración que representa las diferentes categorías.
#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum Category {
    VIP,
    Business,
    Economy,
}

impl Category {
    /// Función para obtener las secciones asociadas a una categoría.
    pub fn sections(&self) -> Vec<Section> {
        match self {
            Category::VIP => vec![Section::A1, Section::B1, Section::C1],
            Category::Business => vec![
                Section::A2, Section::B2, Section::C2,
                Section::A3, Section::B3, Section::C3,
            ],
            Category::Economy => vec![Section::D, Section::E, Section::F],
        }
    }

    /// Función para obtener la categoría inferior.
    pub fn lower_category(&self) -> Option<Category> {
        match self {
            Category::VIP => Some(Category::Business),
            Category::Business => Some(Category::Economy),
            Category::Economy => None,
        }
    }
}

/// Enumeración que representa las diferentes secciones en la disposición de asientos.
#[derive(Debug, Hash, Eq, PartialEq, PartialOrd, Ord, Clone, Copy, Serialize, Deserialize)]
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

    let sections_vip = vec![Section::A1, Section::B1, Section::C1]; // VIP
    let sections_business_1 = vec![Section::A2, Section::B2, Section::C2]; // Business
    let sections_business_2 = vec![Section::A3, Section::B3, Section::C3]; // Business
    let sections_economy = vec![Section::D, Section::E, Section::F]; // Economy

    add_seats(&mut seats, &sections_vip, 1..=2, 1..=5, 100.0, 150.0);
    add_seats(&mut seats, &sections_business_1, 1..=4, 1..=6, 90.0, 90.0);
    add_seats(&mut seats, &sections_business_2, 1..=4, 1..=6, 80.0, 80.0);
    add_seats(&mut seats, &sections_economy, 1..=4, 1..=8, 70.0, 30.0);

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

/// Función para encontrar sugerencias de asientos según las nuevas especificaciones.
pub fn find_seats_suggestions_by_category(
    seats_amount: u32,
    category: Category,
    seats: Arc<Mutex<HashMap<(Section, u32, u32), Seat>>>,
) -> Vec<Vec<(Section, u32, u32)>> {
    // Intentar encontrar sugerencias en la categoría solicitada.
    let mut suggestions = find_seats_in_category(seats_amount, category, Arc::clone(&seats));

    if !suggestions.is_empty() {
        // Si se encontraron sugerencias, retornarlas sin ejecutar métodos alternativos.
        return suggestions;
    } else {
        // Si no se encontraron sugerencias, intentar combinar asientos dentro de la categoría.
        suggestions = find_combined_seats_in_category(seats_amount, category, Arc::clone(&seats));

        if !suggestions.is_empty() {
            return suggestions;
        } else {
            // Si aún no se encontraron sugerencias, buscar en categorías inferiores.
            let mut current_category = category;
            while let Some(lower_category) = current_category.lower_category() {
                current_category = lower_category;
                suggestions = find_seats_in_category(seats_amount, current_category, Arc::clone(&seats));

                if !suggestions.is_empty() {
                    return suggestions;
                }

                // Intentar combinaciones en la categoría inferior.
                suggestions = find_combined_seats_in_category(seats_amount, current_category, Arc::clone(&seats));

                if !suggestions.is_empty() {
                    return suggestions;
                }
            }
        }
    }

    // Si no se encontraron sugerencias en ninguna categoría, retornar un vector vacío.
    suggestions
}

/// Función para encontrar sugerencias de asientos en una categoría específica.
fn find_seats_in_category(
    seats_amount: u32,
    category: Category,
    seats: Arc<Mutex<HashMap<(Section, u32, u32), Seat>>>,
) -> Vec<Vec<(Section, u32, u32)>> {
    let sections = category.sections();

    // Contar los asientos disponibles en cada sección sin mantener el bloqueo
    let section_counts = {
        let seats_guard = seats.lock().unwrap();
        let counts: Vec<(Section, u32)> = sections.iter().map(|&section| {
            let count = seats_guard.iter().filter(|(&(sec, _, _), seat)| {
                sec == section && seat.booked == 'F'
            }).count() as u32;
            (section, count)
        }).collect();
        counts
    };

    // Ordenar las secciones por la mayor cantidad de asientos disponibles
    let mut section_counts = section_counts;
    section_counts.sort_by(|a, b| b.1.cmp(&a.1));

    let mut suggestions = Vec::new();
    let mut visited_sections = Vec::new();

    // Buscar asientos en secciones con más disponibilidad
    for &(section, _) in &section_counts {
        let available_seats = find_seats_by_section(
            seats_amount,
            section,
            Arc::clone(&seats),
            &mut visited_sections,
        );

        if !available_seats.is_empty() {
            suggestions.push(available_seats);
        }

        if suggestions.len() == 3 {
            break;
        }
    }

    // Filtrar sugerencias que tengan la cantidad requerida de asientos
    suggestions.retain(|s| s.len() as u32 == seats_amount);

    suggestions
}

/// Función para combinar asientos de diferentes secciones dentro de una categoría.
fn find_combined_seats_in_category(
    seats_amount: u32,
    category: Category,
    seats: Arc<Mutex<HashMap<(Section, u32, u32), Seat>>>,
) -> Vec<Vec<(Section, u32, u32)>> {
    let sections = category.sections();

    let seats_guard = seats.lock().unwrap();

    let mut all_available_seats = Vec::new();

    // Recolectar todos los asientos disponibles en la categoría
    for &section in &sections {
        for (&(sec, row, number), seat) in seats_guard.iter() {
            if sec == section && seat.booked == 'F' {
                all_available_seats.push((sec, row, number));
            }
        }
    }

    // Ordenar los asientos por sección, fila y número
    all_available_seats.sort();

    // Generar combinaciones posibles
    let mut suggestions = Vec::new();

    // Intentar generar hasta 3 sugerencias
    for _ in 0..3 {
        if all_available_seats.len() as u32 >= seats_amount {
            let suggestion = all_available_seats.iter().take(seats_amount as usize).cloned().collect();
            suggestions.push(suggestion);
            all_available_seats.drain(0..seats_amount as usize);
        } else {
            break;
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
    let mut best_seats_options = Vec::new();

    // Obtener todas las filas en la sección
    let rows: Vec<u32> = seats_guard.keys()
        .filter(|&&(sec, _, _)| sec == section)
        .map(|&(_, row, _)| row)
        .collect();
    let max_row = *rows.iter().max().unwrap_or(&0);

    // Buscar asientos disponibles en la sección
    for current_row in 1..=max_row {
        let mut row_seats = Vec::new();

        // Obtener todos los números de asiento en la fila actual
        let numbers: Vec<u32> = seats_guard.keys()
            .filter(|&&(sec, row, _)| sec == section && row == current_row)
            .map(|&(_, _, number)| number)
            .collect();
        let max_number = *numbers.iter().max().unwrap_or(&0);

        for number in 1..=max_number {
            if let Some(seat) = seats_guard.get(&(section, current_row, number)) {
                if seat.booked == 'F' {
                    // Verificar si el asiento ya está en row_seats
                    if !row_seats.contains(&(section, current_row, number)) {
                        row_seats.push((section, current_row, number));
                    }
                    if row_seats.len() >= seats_amount as usize {
                        break;
                    }
                } else {
                    if !row_seats.is_empty() {
                        best_seats_options.push(row_seats.clone());
                        row_seats.clear();
                    }
                }
            }
        }

        // Si encontramos suficientes asientos en esta fila
        if row_seats.len() >= seats_amount as usize {
            available_seats.extend(row_seats);
            break;
        } else {
            if !row_seats.is_empty() {
                best_seats_options.push(row_seats.clone());
            }
        }
    }

    // Si no se encontraron suficientes asientos en una fila, combinar opciones
    if available_seats.len() < seats_amount as usize {
        for option in best_seats_options {
            for seat in option {
                if !available_seats.contains(&seat) {
                    available_seats.push(seat);
                }
                if available_seats.len() >= seats_amount as usize {
                    break;
                }
            }
            if available_seats.len() >= seats_amount as usize {
                break;
            }
        }
    }

    // Si aún no se alcanzó la cantidad requerida, buscar en filas adyacentes
    if available_seats.len() < seats_amount as usize {
        let additional_seats = find_additional_seats_in_section(
            seats_amount as usize - available_seats.len(),
            section,
            &seats_guard,
            &available_seats,
        );
        available_seats.extend(additional_seats);
    }

    // Truncar la lista de asientos disponibles al número deseado
    available_seats.truncate(seats_amount as usize);
    available_seats
}

/// Función para encontrar asientos adicionales en filas adyacentes dentro de una sección.
fn find_additional_seats_in_section(
    seats_needed: usize,
    section: Section,
    seats_guard: &HashMap<(Section, u32, u32), Seat>,
    existing_seats: &Vec<(Section, u32, u32)>,
) -> Vec<(Section, u32, u32)> {
    let mut additional_seats = Vec::new();

    // Obtener todas las filas y números en la sección
    let mut seats_list: Vec<(u32, u32)> = seats_guard.keys()
        .filter(|&&(sec, _, _)| sec == section)
        .map(|&(_, row, number)| (row, number))
        .collect();

    // Ordenar los asientos por fila y número
    seats_list.sort();

    for &(row, number) in &seats_list {
        if let Some(seat) = seats_guard.get(&(section, row, number)) {
            if seat.booked == 'F' {
                let seat_tuple = (section, row, number);
                if !existing_seats.contains(&seat_tuple) && !additional_seats.contains(&seat_tuple) {
                    additional_seats.push(seat_tuple);
                    if additional_seats.len() >= seats_needed {
                        break;
                    }
                }
            }
        }
    }

    additional_seats
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