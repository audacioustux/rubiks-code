use aoclib::*;
use itertools::*;
use lazy_static::lazy_static;



pub struct Puzzle;

lazy_static! {}

impl InputParsable for Puzzle {
    type Input = Vec<u64>;

    fn parse_input(raw_input: &str) -> Result<Self::Input> {
        Ok(raw_input
            .trim()
            .lines()
            .map(|line| line.trim().parse().unwrap())
            .collect())
    }
}

impl Solvable for Puzzle {
    type Solution1 = u64;
    type Solution2 = u64;

    fn part_one(input: &Self::Input) -> Option<Self::Solution1> {
        Some(Self::find_weakness(input, 25))
    }

    fn part_two(input: &Self::Input) -> Option<Self::Solution1> {
        Some(Self::find_enc_weakness(input, 25))
    }
}

impl Puzzle {
    fn find_weakness(xmas_input: &Vec<u64>, preamble: u8) -> u64 {
        *xmas_input
            .windows(preamble as usize + 1)
            .find_map(|preamble_window| {
                let n = preamble_window.last().unwrap();

                preamble_window[0..preamble as usize]
                    .iter()
                    .combinations(2)
                    .find_map(|preamble_comb| {
                        if preamble_comb[0] + preamble_comb[1] == *n {
                            return Some(());
                        }
                        None
                    })
                    .is_none()
                    .then_some(n)
            })
            .unwrap()
    }

    fn find_enc_weakness(xmas_input: &Vec<u64>, preamble: u8) -> u64 {
        let n = Self::find_weakness(xmas_input, preamble);
        (2..=xmas_input.len())
            .find_map(|window_size| {
                xmas_input
                    .windows(window_size)
                    .map(|window| window.iter())
                    .skip_while(|window| window.as_ref().iter().sum::<u64>() != n)
                    .map(|window| {
                        window.as_ref().iter().min().unwrap()
                            + window.as_ref().iter().max().unwrap()
                    }).next()
            })
            .unwrap()
    }
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
