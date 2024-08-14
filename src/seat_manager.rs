use std::collections::HashMap;

#[derive(Debug)]
pub struct Seat{
    pub number: u32,
    pub section: Section,
    pub row: u32,
    pub visibility: f32,
    pub price: f32,
    pub booked: bool
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub enum Section {
    A1,B1,C1,A2,B2,C2,A3,B3,C3,D,E,F,G,H

}

pub fn create_seats() -> HashMap<(Section, u32, u32),Seat> {
    let mut seats = HashMap::new();

    let sections_abc1 = vec![Section::A1, Section::B1, Section::C1];
    let sections_abc2 = vec![Section::A2, Section::B2, Section::C2];
    let sections_abc3 = vec![Section::A3, Section::B3, Section::C3];
    let sections_defgh = vec![Section::D, Section::E, Section::F, Section::G, Section::H];


    for section in sections_abc1 {
        for row in 1..=2 {
            for number in 1..=5 {
                seats.insert((section.clone(), row, number), Seat{
                    number,
                    section: section.clone(),
                    row,
                    visibility: 100.0,
                    price: 150.0,
                    booked: false
                });
            }
        }
    }

    for section in sections_abc2 {
        for row in 1..=4 {
            for number in 1..=6 {
                seats.insert((section.clone(), row, number), Seat{
                    number,
                    section: section.clone(),
                    row,
                    visibility: 90.0,
                    price: 90.0,
                    booked: false
                });
            }
        }

    }

    for section in sections_abc3 {
        for row in 1..=4 {
            for number in 1..=6 {
                seats.insert((section.clone(), row, number), Seat{
                    number,
                    section: section.clone(),
                    row,
                    visibility: 80.0,
                    price: 80.0,
                    booked: false

                });
            }
        }
    }

    for section in sections_defgh {
        for row in 1..=4 {
            for number in 1..=8 {
                seats.insert((section.clone(), row, number), Seat {
                    number,
                    section: section.clone(),
                    row,
                    visibility: 70.0,
                    price: 30.0,
                    booked: false
                });
            }
        }
    }

    seats
}
