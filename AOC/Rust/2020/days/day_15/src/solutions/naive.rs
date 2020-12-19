use aoclib::*;

pub struct Puzzle;

impl InputParsable for Puzzle {
    type Input = Vec<u32>;

    fn parse_input(raw_input: &str) -> Result<Self::Input> {
        Ok(raw_input
            .trim()
            .split(',')
            .map(|s| s.parse())
            .collect::<Result<Vec<u32>, _>>()?)
    }
}
impl Solvable for Puzzle {
    type Solution1 = u32;

    fn part_one(input: &Self::Input) -> Option<Self::Solution1> {
        Some(Self::memory_game(input, 2020))
    }
    fn part_two(input: &Self::Input) -> Option<Self::Solution1> {
        Some(Self::memory_game(input, 30_000_000))
    }
}

impl Puzzle {
    fn memory_game(init_nums: &[u32], nth: usize) -> u32 {
        let mut turns = vec![0; nth];

        init_nums
            .iter()
            .enumerate()
            .for_each(|(i, n)| turns[*n as usize] = i + 1);

        (init_nums.len()..turns.capacity()).fold(
            *init_nums.last().unwrap() as usize,
            |last_spoke, turn| {
                let last_spoke_at = turns[last_spoke];
                turns[last_spoke] = turn;

                if last_spoke_at == 0 {
                    return turn - last_spoke_at;
                }
                0
            },
        ) as u32
    }
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
