#![feature(associated_type_defaults)]

use std::error::Error;
use std::fmt::Display;

pub type BoxError = Box<dyn Error + Send + Sync>;
pub type Result<T, E = BoxError> = std::result::Result<T, E>;

pub trait InputParsable {
    type Input;
    fn parse_input(_: &str) -> Result<Self::Input>;
}

pub trait Solvable: InputParsable {
    type Solution1: Display = u32;
    type Solution2: Display = u32;

    fn part_one(_input: &Self::Input) -> Option<Self::Solution1> {
        None
    }

    fn part_two(_input: &Self::Input) -> Option<Self::Solution2> {
        None
    }

    fn solve(raw_input: &str) -> Result<(Option<Self::Solution1>, Option<Self::Solution2>)> {
        let input = Self::parse_input(raw_input)?;
        Ok((Self::part_one(&input), Self::part_two(&input)))
    }
}
