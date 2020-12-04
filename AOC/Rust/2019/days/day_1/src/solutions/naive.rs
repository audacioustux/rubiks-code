use aoclib::*;

pub struct Puzzle;

impl InputParsable for Puzzle {
    type Input = Vec<u32>;
    fn parse_input(raw_input: &str) -> Result<Self::Input> {
        Ok(raw_input
            .trim()
            .lines()
            .map(|line| line.trim())
            .map(|line| line.parse::<u32>())
            .collect::<Result<_, _>>()?)
    }
}

impl Solvable for Puzzle {
    type Solution1 = u32;
    type Solution2 = u32;

    fn part_one(input: &Self::Input) -> Option<Self::Solution1> {
        Some(
            input
                .iter()
                .fold(0, |acc, module_mass| acc + fuel(*module_mass)),
        )
    }

    fn part_two(input: &Self::Input) -> Option<Self::Solution2> {
        Some(
            input
                .iter()
                .fold(0, |acc, module_mass| acc + fuel_recursive(*module_mass)),
        )
    }
}

fn fuel(payload_mass: u32) -> u32 {
    match (payload_mass / 3).checked_sub(2) {
        Some(m) => m,
        None => 0,
    }
}

fn fuel_recursive(payload_mass: u32) -> u32 {
    match fuel(payload_mass) {
        0 => 0,
        fuel_mass => fuel_mass + fuel_recursive(fuel_mass),
    }
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
