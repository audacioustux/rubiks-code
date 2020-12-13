use aoclib::*;

pub struct Puzzle;

impl InputParsable for Puzzle {
    type Input = Vec<u64>;
    fn parse_input(raw_input: &str) -> Result<Self::Input> {
        let mut input: Self::Input = raw_input
            .trim()
            .lines()
            .map(|line| line.trim().to_string())
            .map(|line| line.parse().unwrap())
            .collect();

        input.push(0);
        input.sort_unstable();
        input.push(input.last().unwrap() + 3);

        Ok(input)
    }
}

impl Solvable for Puzzle {
    type Solution2 = u64;
    type Solution1 = u64;

    fn part_one(input: &Self::Input) -> Option<Self::Solution1> {
        let diffs = Self::joltage_differences(input)
            .iter()
            .skip(1)
            .fold([0, 0, 0], |acc, diff| {
                let mut acc = acc;
                acc[*diff as usize - 1] += 1;
                acc
            });

        Some(diffs[0] * diffs[2])
    }

    fn part_two(input: &Self::Input) -> Option<Self::Solution2> {
        Some(Self::count_possibilities(&input))
    }
}

impl Puzzle {
    fn joltage_differences(adapters: &Vec<u64>) -> Vec<u64> {
        adapters
            .iter()
            .scan(0, |acc, joltage| {
                let diff = *joltage - *acc;
                *acc = *joltage;

                Some(diff)
            })
            .collect()
    }

    // NOTE: thanks to https://github.com/tudorpavel/advent-of-code-2020/blob/master/day10/README.md
    fn count_possibilities(adapters: &Vec<u64>) -> u64 {
        let mut slices = vec![];
        let mut current_slice = vec![];

        for window in adapters.windows(2) {
            match window[1] - window[0] {
                1 => current_slice.push(window[0]),
                3 => {
                    current_slice.push(window[0]);
                    slices.push(current_slice);
                    current_slice = vec![];
                }
                _ => (),
            }
        }

        slices
            .iter()
            .map(|slice| match slice.len() {
                1 => 1,
                2 => 1,
                3 => 2,
                4 => 4,
                5 => 7,
                _ => panic!("unexpected slice of size N > 5 consecutive 1-diff elements"),
            })
            .product()
    }
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
