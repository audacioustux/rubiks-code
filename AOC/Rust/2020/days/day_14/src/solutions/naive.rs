use aoclib::*;
use lazy_static::lazy_static;
use std::collections::HashMap;

pub struct Puzzle;

lazy_static! {}

#[derive(Debug, Clone, Copy)]
pub struct BitMask {
    and: u64,
    or: u64,
}

impl Default for BitMask {
    fn default() -> Self {
        Self {
            and: 2u64.pow(36) - 1,
            or: 0,
        }
    }
}

impl From<&str> for BitMask {
    fn from(raw_mask: &str) -> Self {
        Self {
            and: u64::from_str_radix(&raw_mask.replace("X", "1"), 2).unwrap(),
            or: u64::from_str_radix(&raw_mask.replace("X", "0"), 2).unwrap(),
        }
    }
}

#[derive(Debug)]
pub enum Instruction {
    Mask(BitMask),
    Memory(u64, u64), // (address, value)
}

impl InputParsable for Puzzle {
    type Input = Vec<Instruction>;

    fn parse_input(input: &str) -> Result<Self::Input> {
        Ok(input
            .trim()
            .lines()
            .map(|l| l.trim())
            .map(|l| {
                let mut split = l.split(" = ");
                let first = split.next().unwrap();

                match first {
                    "mask" => Instruction::Mask(BitMask::from(split.next().unwrap())),
                    _ => {
                        let address = first[4..first.len() - 1].parse::<u64>().unwrap();
                        let value = split.next().unwrap().parse::<u64>().unwrap();
                        Instruction::Memory(address, value)
                    }
                }
            })
            .collect())
    }
}

impl Solvable for Puzzle {
    type Solution1 = u64;
    type Solution2 = u64;

    fn part_one(input: &Self::Input) -> Option<Self::Solution1> {
        let mut current_mask = BitMask::default();
        let mut memory = HashMap::new();

        for instruction in input {
            match instruction {
                Instruction::Mask(mask) => current_mask = *mask,
                Instruction::Memory(address, value) => {
                    memory.insert(*address, value & current_mask.and | current_mask.or);
                }
            }
        }

        Some(memory.values().sum())
    }

    fn part_two(input: &Self::Input) -> Option<Self::Solution2> {
        let mut current_mask = BitMask::default();
        let mut x_mask = 0;
        let mut memory = HashMap::new();

        for instruction in input {
            match instruction {
                Instruction::Mask(mask) => {
                    current_mask = *mask;
                    x_mask = current_mask.and ^ current_mask.or
                }
                Instruction::Memory(address, value) => {
                    let mut address_base = address | current_mask.or;
                    let mut x_mask = x_mask;

                    let mut addresses: Vec<u64> = Vec::new();

                    for i in 0..36 {
                        let a = address_base % 2;
                        let x = x_mask % 2;

                        addresses = if x == 1 {
                            if addresses.is_empty() {
                                vec![0, 1]
                            } else {
                                addresses
                                    .iter()
                                    .map(|address| vec![address + 2_u64.pow(i as u32), *address])
                                    .flatten()
                                    .collect()
                            }
                        } else if addresses.is_empty() {
                            vec![a]
                        } else {
                            addresses
                                .iter()
                                .map(|address| address + a * 2_u64.pow(i as u32))
                                .collect()
                        };

                        address_base >>= 1;
                        x_mask >>= 1;
                    }

                    for address in addresses {
                        memory.insert(address, *value);
                    }
                }
            }
        }

        Some(memory.values().sum())
    }
}

impl Puzzle {}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
