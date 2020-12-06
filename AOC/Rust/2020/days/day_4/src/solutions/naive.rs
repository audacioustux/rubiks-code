use aoclib::*;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::default::Default;
use std::ops::{Index, IndexMut};
use std::str::FromStr;

pub struct Puzzle;

#[derive(Debug, Default)]
pub struct Passport {
    birth_year: Option<String>,
    issue_year: Option<String>,
    expiration_year: Option<String>,
    height: Option<String>,
    hair_color: Option<String>,
    eye_color: Option<String>,
    passport_id: Option<String>,
    country_id: Option<String>,
}
impl Passport {
    fn new(input: &str) -> Option<Self> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(?P<field>\w+):(?P<value>\S*)").unwrap();
        }
        let mut passport = Passport::default();

        RE.captures_iter(input).for_each(|m| {
            let value = Some(m["value"].to_string());
            match &m["field"] {
                "byr" => passport.birth_year = value,
                "iyr" => passport.issue_year = value,
                "eyr" => passport.expiration_year = value,
                "hgt" => passport.height = value,
                "hcl" => passport.hair_color = value,
                "ecl" => passport.eye_color = value,
                "pid" => passport.passport_id = value,
                "cid" => passport.country_id = value,
                _ => unreachable!(),
            };
        });

        Some(passport)
    }
}

impl InputParsable for Puzzle {
    type Input = Vec<Passport>;

    fn parse_input(raw_input: &str) -> Result<Self::Input> {
        Ok(raw_input
            .trim() // santize input
            .lines()
            .map(|line| line.trim().to_string()) // sanitize incidents
            .group_by(|line| line.is_empty()) // group passport data, seperated by blank line
            .into_iter()
            .map(|(_, mut lines)| lines.join(" "))
            .filter(|lines| !lines.is_empty())
            .filter_map(|passport_lines| Passport::new(&passport_lines))
            .collect())
    }
}

impl Solvable for Puzzle {
    fn part_one(input: &Self::Input) -> Option<Self::Solution1> {
        Some(
            input
                .iter()
                .inspect(|x| {
                    dbg!(x);
                })
                .filter(|passport| {
                    [
                        &passport.birth_year,
                        &passport.issue_year,
                        &passport.expiration_year,
                        &passport.height,
                        &passport.hair_color,
                        &passport.eye_color,
                        &passport.passport_id,
                    ]
                    .iter()
                    .all(|v| v.is_some())
                })
                .count() as u32,
        )
    }
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
