mod literal;
pub use literal::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Date {
    year: u16,
    month: u8,
    day: u8,
}

impl Date {
    pub fn new(year: u16, month: u8, day: u8) -> Date {
        Date { year, month, day }
    }
}
