mod literal;

use std::fmt::Display;

pub use literal::*;
use std::num::ParseIntError;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq)]
pub struct Date {
    year: u16,
    month: u8,
    day: u8,
}

#[derive(Debug, Clone, Error, PartialEq)]
pub enum DateError {
    #[error("{0} is out of range!")]
    DateOutOfRange(Date),
    #[error(transparent)]
    ParseIntError(#[from] ParseIntError),
}

impl Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.year, self.month, self.day)
    }
}

impl Date {
    pub fn unchecked(year: u16, month: u8, day: u8) -> Date {
        Date { year, month, day }
    }

    pub fn year(&self) -> u16 {
        self.year
    }
    pub fn month(&self) -> u8 {
        self.month
    }
    pub fn day(&self) -> u8 {
        self.day
    }

    pub fn parse(date: [&str; 3]) -> Result<Self, DateError> {
        let date = Date::unchecked(date[0].parse()?, date[1].parse()?, date[2].parse()?);

        if date.month() > 12 || date.month() < 1 || date.day() < 1 || date.day() > 31 {
            return Err(DateError::DateOutOfRange(date));
        }

        Ok(date)
    }
}
