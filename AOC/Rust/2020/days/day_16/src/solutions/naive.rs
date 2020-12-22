use aoclib::*;
use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    iter,
    ops::RangeInclusive,
};

pub struct Puzzle;

type Rules = Vec<(String, Vec<RangeInclusive<u32>>)>;
type Ticket = Vec<u32>;
type NearbyTickets = Vec<Ticket>;

impl InputParsable for Puzzle {
    type Input = (Rules, Ticket, NearbyTickets);

    fn parse_input(raw_input: &str) -> Result<Self::Input> {
        let (rules, ticket, nearby_tickets) =
            raw_input.trim().split("\n\n").collect_tuple().unwrap();

        let rules: Rules = rules
            .lines()
            .map(|r_str| r_str.split(':').collect_tuple().unwrap()) // parse `field: xx-xx or yy-yy`
            .map(|(rule_field, ranges)| {
                (
                    rule_field.to_string(),
                    ranges
                        .trim()
                        .split("or") // parse `22-22 or 24-24`
                        .map(|range_str| {
                            range_str
                                .trim()
                                .split('-') // parse `22-22`
                                .map(|n_str| n_str.parse().unwrap())
                                .collect_tuple()
                                .unwrap()
                        })
                        .map(|(from, to)| from..=to)
                        .collect(),
                )
            })
            .collect();

        let parse_ticket =
            |t_str: &str| -> Ticket { t_str.split(',').map(|n| n.parse().unwrap()).collect() };

        let ticket = ticket.lines().skip(1).map(parse_ticket).next().unwrap();

        let nearby_tickets = nearby_tickets.lines().skip(1).map(parse_ticket).collect();

        return Ok((rules, ticket, nearby_tickets));
    }
}

impl Solvable for Puzzle {
    type Solution2 = u64;

    fn part_one(input: &Self::Input) -> Option<Self::Solution1> {
        let (rules, _, nearby_tickets) = input;

        let mut error_rate = 0;
        nearby_tickets.iter().for_each(|ticket| {
            ticket.iter().for_each(|value| {
                if !Self::is_valid_value(rules, value) {
                    error_rate += value;
                }
            })
        });

        Some(error_rate)
    }

    fn part_two(input: &Self::Input) -> Option<Self::Solution2> {
        let (rules, ticket, nearby_tickets) = input;

        let valid_tickets = nearby_tickets
            .iter()
            .filter(|ticket| {
                ticket
                    .iter()
                    .all(|value| Self::is_valid_value(rules, value))
            })
            .chain(iter::once(ticket))
            .collect_vec();

        let mut unknown: HashMap<String, HashSet<usize>> = rules
            .iter()
            .map(|rule| {
                (
                    rule.0.clone(),
                    (0..ticket.len())
                        .filter(|i| {
                            valid_tickets.iter().all(|nearby_ticket| {
                                rule.1
                                    .iter()
                                    .any(|range| range.contains(&nearby_ticket[*i]))
                            })
                        })
                        .collect(),
                )
            })
            .collect();

        let mut known = HashMap::<String, usize>::new();

        while !unknown.is_empty() {
            let (field, i) = {
                unknown
                    .iter()
                    .filter(|(_, i_set)| i_set.len() == 1)
                    .map(|(field, i)| (field.clone(), *i.iter().next().unwrap()))
                    .next()
                    .unwrap()
            };

            unknown.remove(&field);
            known.insert(field, i);

            for i_set in unknown.values_mut() {
                i_set.remove(&i);
            }
        }

        Some(
            known
                .iter()
                .filter(|(field, _)| field.starts_with("departure"))
                .map(|(_, i)| ticket[*i] as u64)
                .filter(|v| v != &0)
                .product(),
        )
    }
}

impl Puzzle {
    fn is_valid_value(rules: &Rules, value: &u32) -> bool {
        rules
            .iter()
            .map(|(_, rule_ranges)| rule_ranges)
            .any(|rule_ranges| rule_ranges.iter().any(|range| range.contains(value)))
    }
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
