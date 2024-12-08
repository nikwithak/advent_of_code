use std::num::ParseIntError;
pub mod util;

pub mod day_01;
pub mod day_02;
pub mod day_03;
pub mod day_04;
pub mod day_05;
pub mod day_06;
pub mod day_07;

#[derive(Debug)]
pub enum Error {
    String(String),
    ParseError(ParseIntError),
}

impl<T: Into<String>> From<T> for Error {
    fn from(s: T) -> Self {
        Self::String(s.into())
    }
}
