mod literal;

use std::fmt::Display;

pub use literal::*;
use std::num::ParseIntError;
use thiserror::Error;

/// A structure representing a date with year, month, and day components.
///
/// The `Date` struct includes fields for the year, month, and day, as well as a boolean
/// indicating whether the date components are zero-padded. It provides methods for
/// creating, parsing, and formatting dates, as well as checking and setting the zero-padding.
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Date {
    year: u16,
    month: u8,
    day: u8,
    zero_padded: bool,
}

#[derive(Debug, Clone, Error, PartialEq)]
pub enum ParseDateError {
    #[error("{0} is defined with too many characters in day or month")]
    TooManyCharacters(Date),
    #[error("{0} is out of range")]
    DateOutOfRange(Date),
    #[error("{0} has too many zero's padded at day or month")]
    InvalidPrefix(Date),
    #[error(transparent)]
    ParseIntError(#[from] ParseIntError),
}

impl Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_zero_padded() {
            write!(
                f,
                "{:04}.{:02}.{:02}",
                self.year(),
                self.month(),
                self.day()
            )
        } else {
            write!(f, "{:04}.{}.{}", self.year(), self.month(), self.day())
        }
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

    /// Parses a date string in the format "YYYY.MM.DD".
    ///
    /// # Panics
    ///
    /// This function will panic if the input string is not in the format "YYYY.MM.DD".
    /// Specifically, it expects the string to contain exactly two dots separating the year,
    /// month, and day components. If the string does not contain exactly two dots, or if any
    /// of the components cannot be parsed into their respective integer types, the function
    /// will panic.
    ///
    /// # Examples
    ///
    /// ```
    /// use pdxsyn::Date;
    /// let date = Date::parse_string_unwrapped("2023.10.05");
    /// assert_eq!(date.year(), 2023);
    /// assert_eq!(date.month(), 10);
    /// assert_eq!(date.day(), 5);
    /// ```
    ///
    /// ```should_panic
    /// use pdxsyn::Date;
    /// // This will panic because the format is incorrect (missing dots)
    /// let date = Date::parse_string_unwrapped("20231005");
    /// ```
    ///
    /// ```
    /// use pdxsyn::Date;
    /// // This will not panic because the parser ignores anything after the 3rd dot
    /// let date = Date::parse_string_unwrapped("2023.10.05.01");
    /// ```
    ///
    /// ```should_panic
    /// use pdxsyn::Date;
    /// // This will panic because the month component is not a valid integer
    /// let date = Date::parse_string_unwrapped("2023.XX.05");
    /// ```
    pub fn parse_string_unwrapped(str: &str) -> Self {
        let mut ymd = str.split(".");
        Self::parse([
            ymd.next().unwrap(),
            ymd.next().unwrap(),
            ymd.next().unwrap(),
        ])
        .unwrap()
    }

    /// Parses a date from an array of string slices representing the year, month, and day.
    ///
    /// # Returns
    ///
    /// * `Ok(Date)` - If the date is successfully parsed and valid.
    /// * `Err(ParseDateError)` if:
    ///     * The year, month, or day strings cannot be parsed into integers.
    ///     * The month is not between 1 and 12.
    ///     * The day is not between 1 and 31.
    ///     * The zero padding in the month or day is invalid.
    ///
    /// # Examples
    ///
    /// ```
    /// use pdxsyn::{Date, ParseDateError};
    /// let date = Date::parse(["2023", "10", "05"]).unwrap();
    /// assert_eq!(date.year(), 2023);
    /// assert_eq!(date.month(), 10);
    /// assert_eq!(date.day(), 5);
    /// assert!(date.is_zero_padded());
    /// ```
    ///
    /// ```
    /// use pdxsyn::{Date, ParseDateError};
    /// let date = Date::parse(["2023", "7", "5"]).unwrap();
    /// assert_eq!(date.year(), 2023);
    /// assert_eq!(date.month(), 7);
    /// assert_eq!(date.day(), 5);
    /// assert!(!date.is_zero_padded());
    /// ```
    ///
    /// ```
    /// use pdxsyn::{Date, ParseDateError};
    /// let result = Date::parse(["2023", "13", "05"]);
    /// assert!(matches!(result, Err(ParseDateError::DateOutOfRange(_))));
    /// ```
    ///
    /// ```
    /// use pdxsyn::{Date, ParseDateError};
    /// let result = Date::parse(["2023", "10", "32"]);
    /// assert!(matches!(result, Err(ParseDateError::DateOutOfRange(_))));
    /// ```
    ///
    /// ```
    /// use pdxsyn::{Date, ParseDateError};
    /// let result = Date::parse(["2023", "010", "05"]);
    /// assert!(matches!(result, Err(ParseDateError::TooManyCharacters(_))));
    /// ```
    pub fn parse([year_str, month_str, day_str]: [&str; 3]) -> Result<Self, ParseDateError> {
        let mut date = Date::unchecked(
            year_str.parse()?,
            month_str.parse()?,
            day_str.parse()?,
            false,
        );

        if month_str.len() > 2 || day_str.len() > 2 {
            return Err(ParseDateError::TooManyCharacters(date));
        }

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
