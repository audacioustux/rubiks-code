use itertools::*;
use std::collections::HashSet;

use aoclib::*;

pub struct Puzzle;

impl InputParsable for Puzzle {
    type Input = Vec<String>;

    fn parse_input(input: &str) -> Result<Self::Input> {
        let input = input
            .trim()
            .lines()
            .map(|s| s.trim())
            .group_by(|line| line.is_empty())
            .into_iter()
            .filter(|(seperator, _)| !*seperator)
            .map(|(_, group)| join(group, " "))
            .collect();

        Ok(input)
    }
}

impl Solvable for Puzzle {
    type Solution1 = usize;
    type Solution2 = usize;

    fn part_one(input: &Self::Input) -> Option<Self::Solution1> {
        Some(
            input
                .iter()
                .map(|v| {
                    v.chars()
                        .filter(|c| !char::is_whitespace(*c))
                        .collect::<HashSet<char>>()
                })
                .map(|h| h.len())
                .sum(),
        )
    }
    fn part_two(input: &Self::Input) -> Option<Self::Solution2> {
        Some(
            input
                .iter()
                .map(|group| {
                    group
                        .split_whitespace()
                        .map(|p_ans| p_ans.chars().collect::<HashSet<char>>())
                        .fold_first(|prev_common_answers, answers| {
                            prev_common_answers
                                .intersection(&answers)
                                .map(|c| *c)
                                .collect()
                        })
                })
                .map(|common_answers| common_answers.unwrap().len())
                .sum(),
        )
    }
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
