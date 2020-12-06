use aoclib::*;

pub struct Puzzle;

impl InputParsable for Puzzle {
    type Input = Vec<String>;
    fn parse_input(raw_input: &str) -> Result<Self::Input> {
        Ok(raw_input
            .trim()
            .lines()
            .map(|line| line.trim().to_string())
            .collect())
    }
}

impl Solvable for Puzzle {
    type Solution1 = u64;
    type Solution2 = u64;

    fn part_one(input: &Self::Input) -> Option<Self::Solution1> {
        Some(check_slope(input, 3, 1))
    }

    fn part_two(input: &Self::Input) -> Option<Self::Solution2> {
        Some(
            [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
                .iter()
                .map(|slope_meta| check_slope(input, slope_meta.0, slope_meta.1))
                .product(),
        )
    }
}

fn check_slope(input: &Vec<String>, right: u8, down: u8) -> u64 {
    input
        .iter()
        .step_by(down as usize)
        .map(|row| row.chars().cycle())
        .zip((0..).step_by(right as usize))
        .map(|(mut row, column)| row.nth(column).unwrap())
        .map(|marker| match marker {
            '#' => true,
            '.' => false,
            _ => unreachable!(),
        })
        .filter(|encounters| *encounters)
        .count() as u64
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
