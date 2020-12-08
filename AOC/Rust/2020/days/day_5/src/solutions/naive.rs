use std::ops::RangeInclusive;

use aoclib::*;

pub struct Puzzle;

struct Seat {
    row: u32,
    col: u32,
}
impl InputParsable for Puzzle {
    type Input = Vec<u32>;

    fn parse_input(input: &str) -> Result<Self::Input> {
        let mut input = input
            .trim()
            .lines()
            .map(|s| s.trim())
            .map(|s| s.split_at(7))
            .map(find_seat)
            .map(|seat| seat.row * 8 + seat.col)
            .collect::<Self::Input>();

        input.sort();

        Ok(input)
    }
}

impl Solvable for Puzzle {
    type Solution1 = u32;

    fn part_one(input: &Self::Input) -> Option<Self::Solution1> {
        input.iter().max().map(|n| *n)
    }
    fn part_two(input: &Self::Input) -> Option<Self::Solution2> {
        input
            .iter()
            .zip(input.iter().skip(1))
            .filter(|(a, b)| *b - *a != 1)
            .map(|(_, b)| *b - 1)
            .next()
    }
}

fn bsp(range: RangeInclusive<u32>, instructions: &str) -> u32 {
    instructions
        .chars()
        .fold(range, |curr, instruction| {
            let (start, end) = (*curr.start(), *curr.end());
            let get_mid = || ((end - start) as f32 / 2.0).round() as u32;

            match instruction {
                'F' | 'L' => start..=end - get_mid(),
                'B' | 'R' => start + get_mid()..=end,
                _ => unreachable!(),
            }
        })
        .next()
        .unwrap()
}
fn find_seat(boarding_pass: (&str, &str)) -> Seat {
    Seat {
        row: bsp(0..=127, boarding_pass.0),
        col: bsp(0..=7, boarding_pass.1),
    }
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
