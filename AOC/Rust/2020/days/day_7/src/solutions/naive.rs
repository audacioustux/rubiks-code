use aoclib::*;
use itertools::*;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

pub struct Puzzle;

lazy_static! {
    static ref CONTAINER_RE: Regex = Regex::new(r"^(?P<color>.*) bags").unwrap();
    static ref CONTAINS_RE: Regex = Regex::new(r"^(?P<amount>\d+) (?P<color>.*) bags?\.?").unwrap();
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Bag {
    color: String,
    amount: u32,
}
impl InputParsable for Puzzle {
    type Input = HashMap<String, Vec<Bag>>;
    fn parse_input(raw_input: &str) -> Result<Self::Input> {
        let input = raw_input
            .trim()
            .lines()
            .map(|line| line.trim())
            .map(|line| line.splitn(2, "contain").collect_tuple().unwrap())
            .fold(
                Self::Input::new(),
                |mut rules, (container_str, contains_str)| {
                    let m = CONTAINER_RE.captures(container_str).unwrap();
                    let container_bag_color = &m["color"];

                    let bags = {
                        let contains_strs: Vec<&str> = contains_str.split(',').collect();
                        contains_strs.iter().map(|s| s.trim_start()).fold(
                            Vec::with_capacity(contains_str.len()),
                            |mut acc_map, curr| {
                                if let Some(m) = CONTAINS_RE.captures(curr) {
                                    let (amount, color) = (&m["amount"], &m["color"]);

                                    acc_map.push(Bag {
                                        amount: amount.parse().unwrap(),
                                        color: color.to_owned(),
                                    });
                                }
                                acc_map
                            },
                        )
                    };

                    rules.insert(container_bag_color.to_owned(), bags);
                    rules
                },
            );

        Ok(input)
    }
}

impl Solvable for Puzzle {
    fn part_one(input: &Self::Input) -> Option<Self::Solution1> {
        Some(find_bag_ancestors(input, &"shiny gold").len() as u32)
    }
    fn part_two(input: &Self::Input) -> Option<Self::Solution2> {
        Some(find_cost(input, &"shiny gold"))
    }
}

// TODO: find a better name xD
fn find_bag_ancestors<'a>(
    haystack: &'a HashMap<String, Vec<Bag>>,
    color: &'a str,
) -> HashSet<&'a String> {
    haystack
        .iter()
        .filter(|(_, contains)| contains.iter().any(|b| b.color == color)) // filter bags that contain `color` type bag
        .fold(HashSet::new(), |mut acc_set, (color, _)| {
            acc_set.insert(color);
            let ancestor = HashSet::from(find_bag_ancestors(&haystack, color));
            ancestor.union(&acc_set).map(|x| *x).collect()
        })
}

fn find_cost(haystack: &HashMap<String, Vec<Bag>>, color: &str) -> u32 {
    if let Some(contains) = haystack.get(color) {
        return contains.iter().fold(0, |acc_cost, Bag { amount, color }| {
            amount + acc_cost + amount * find_cost(haystack, color)
        });
    }
    1
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
