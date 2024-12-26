use core::{fmt::Display, num::ParseIntError};

#[derive(Debug)]
pub enum Error {
    InvalidProblem,
    ParseIntError(ParseIntError),
}
impl Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidProblem => write!(f, "Solution::Error::InvalidProblem"),
            Self::ParseIntError(e) => write!(f, "{}", e),
        }
    }
}
impl From<core::num::ParseIntError> for Error {
    fn from(val: core::num::ParseIntError) -> Self {
        Self::ParseIntError(val)
    }
}

pub trait Solution: core::fmt::Debug + Default {
    type ResultType: core::fmt::Debug + Display;
    type ParseError: core::fmt::Debug + Display;

    fn update_from_line(&mut self, _id: usize, _line: &str) -> Result<(), Self::ParseError> {
        Ok(())
    }

    fn analyse(&mut self, is_full: bool);

    fn answer_part1(&self, is_full: bool) -> Result<Self::ResultType, Error>;
    fn answer_part2(&self, is_full: bool) -> Result<Self::ResultType, Error>;
}
