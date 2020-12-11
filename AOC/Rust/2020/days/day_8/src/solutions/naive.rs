use aoclib::*;
use itertools::*;
use lazy_static::lazy_static;
use std::collections::HashSet;
use std::str::FromStr;

pub struct Puzzle;

lazy_static! {}

impl InputParsable for Puzzle {
    type Input = Vec<Instruction>;

    fn parse_input(raw_input: &str) -> Result<Self::Input> {
        let input = raw_input
            .trim()
            .lines()
            .map(|line| line.trim())
            .map(|line| line.splitn(2, ' ').collect_tuple().unwrap())
            .map(|(op, arg)| Instruction {
                op: op.parse().unwrap(),
                arg: arg.parse().unwrap(),
            })
            .collect();

        Ok(input)
    }
}

impl Solvable for Puzzle {
    type Solution1 = i32;
    type Solution2 = i32;

    fn part_one(input: &Self::Input) -> Option<Self::Solution1> {
        Some(
            State {
                acc: 0,
                ip: 0,
                exit_state: None,
            }
            .derive_final_state(input)
            .acc,
        )
    }

    fn part_two(input: &Self::Input) -> Option<Self::Solution2> {
        use Operation::*;

        input
            .iter()
            .enumerate()
            .filter(|(_, ins)| matches!(ins.op, Nop | Jmp))
            .find_map(|(ip, ins)| {
                let mut new_input: Self::Input = input.iter().copied().collect();

                let op = match ins.op {
                    Jmp => Nop,
                    Nop => Jmp,
                    _ => unreachable!(),
                };

                new_input[ip] = Instruction { op, ..*ins };

                let final_state = State {
                    acc: 0,
                    ip: 0,
                    exit_state: None,
                }
                .derive_final_state(&new_input);

                if let Some(ExitState::Terminated) = final_state.exit_state {
                    return Some(final_state.acc);
                }
                None
            })
    }
}

#[derive(Debug, Copy, Clone)]
enum Operation {
    Acc,
    Jmp,
    Nop,
}
impl FromStr for Operation {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "acc" => Self::Acc,
            "jmp" => Self::Jmp,
            "nop" => Self::Nop,
            _ => unreachable!(),
        })
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Instruction {
    op: Operation,
    arg: i32,
}

#[derive(Debug, Copy, Clone)]
enum ExitState {
    Cyclic,
    Terminated,
}

#[derive(Debug, Copy, Clone)]
struct State {
    acc: i32,
    ip: usize,
    exit_state: Option<ExitState>,
}

impl State {
    fn exec(self, ins: &Instruction) -> State {
        match ins.op {
            Operation::Acc => State {
                acc: self.acc + ins.arg,
                ip: self.ip + 1,
                ..self
            },
            Operation::Jmp => State {
                ip: (self.ip as i32 + ins.arg) as usize,
                ..self
            },
            Operation::Nop => State {
                ip: self.ip + 1,
                ..self
            },
        }
    }

    fn derive_final_state(mut self, instructions: &Vec<Instruction>) -> State {
        let mut visited: HashSet<usize> = HashSet::new();

        loop {
            visited.insert(self.ip);

            if let Some(ins) = instructions.get(self.ip) {
                let new_state = self.exec(ins);

                if visited.get(&new_state.ip).is_some() {
                    break State {
                        exit_state: Some(ExitState::Cyclic),
                        ..new_state
                    };
                }

                self = new_state;
            } else {
                break Self {
                    exit_state: Some(ExitState::Terminated),
                    ..self
                };
            }
        }
    }
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
