mod literal;

use std::fmt::Display;

pub use literal::*;
use std::num::ParseIntError;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Date {
    year: u16,
    month: u8,
    day: u8,
    zero_padded: bool,
}

#[derive(Debug, Clone, Error, PartialEq)]
pub enum ParseDateError {
    #[error("{0} is out of range")]
    DateOutOfRange(Date),
    #[error("{0} has too many zero's padded at day or month")]
    InvalidPrefix(Date),
    #[error(transparent)]
    ParseIntError(#[from] ParseIntError),
}

impl Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.year, self.month, self.day)
    }
}

impl Date {
    #[inline]
    pub fn unchecked(year: u16, month: u8, day: u8, zero_padded: bool) -> Date {
        Date {
            year,
            month,
            day,
            zero_padded,
        }
    }

    #[inline]
    pub fn year(&self) -> u16 {
        self.year
    }

    #[inline]
    pub fn month(&self) -> u8 {
        self.month
    }

    #[inline]
    pub fn day(&self) -> u8 {
        self.day
    }

    #[inline]
    pub fn set_zero_padded(&mut self, zero_padded: bool) {
        self.zero_padded = zero_padded;
    }

    #[inline]
    pub fn is_zero_padded(&self) -> bool {
        self.zero_padded
    }

    pub fn parse([year_str, month_str, day_str]: [&str; 3]) -> Result<Self, ParseDateError> {
        let mut date = Date::unchecked(
            year_str.parse()?,
            month_str.parse()?,
            day_str.parse()?,
            false,
        );

        let month_pad_diff = month_str.len() - date.month().to_string().len();
        let day_pad_diff = day_str.len() - date.day().to_string().len();
        match (month_pad_diff, day_pad_diff) {
            (0, 0) => date.set_zero_padded(false),
            (1, 0..=1) => date.set_zero_padded(true),
            (0..=1, 1) => date.set_zero_padded(true),
            _ => return Err(ParseDateError::InvalidPrefix(date)),
        }

        if date.month() > 12 || date.month() < 1 || date.day() < 1 || date.day() > 31 {
            return Err(ParseDateError::DateOutOfRange(date));
        }

        Ok(date)
    }
}
