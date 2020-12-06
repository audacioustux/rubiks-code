use aoclib::*;
use itertools::Itertools;
use regex::Regex;
use std::str::FromStr;

pub struct Puzzle;

#[derive(Debug)]
pub enum PassportField {
    BirthYear(String),
    IssueYear(String),
    ExpirationYear(String),
    Height(String),
    HairColor(String),
    EyeColor(String),
    PassportId(String),
    CountryId(String),
}
impl FromStr for PassportField {
    type Err = ();

    fn from_str(input: &str) -> Result<PassportField, Self::Err> {
        use PassportField::*;

        let re = Regex::new(r"^(?P<field>\w+):(?P<value>\S*)$").or(Err(()))?;
        let captures = re.captures(input).ok_or(())?;

        let (field, value) = (&captures["field"], captures["value"].to_string());
        match field {
            "byr" => Ok(BirthYear(value)),
            "iyr" => Ok(IssueYear(value)),
            "eyr" => Ok(ExpirationYear(value)),
            "hgt" => Ok(Height(value)),
            "hcl" => Ok(HairColor(value)),
            "ecl" => Ok(EyeColor(value)),
            "pid" => Ok(PassportId(value)),
            "cid" => Ok(CountryId(value)),
            _ => Err(()),
        }
    }
}

type Passport = Vec<PassportField>;

impl InputParsable for Puzzle {
    type Input = Vec<Passport>;

    fn parse_input(raw_input: &str) -> Result<Self::Input> {
        Ok(raw_input
            .trim() // santize input
            .lines()
            .map(|line| line.trim().to_string()) // sanitize incidents
            .group_by(|line| line != "") // group passport data, seperated by blank line
            .into_iter()
            .filter(|(is_passport, _)| *is_passport) // filter out seperator lines
            .map(|(_, lines)| lines) // only keep raw passport data
            .map(|mut passports_raw| {
                passports_raw // parse to `Passport`
                    .join(" ") // merge all fields on a single line
                    .split(" ") // get all the field:value pairs
                    .map(|fields_raw| PassportField::from_str(fields_raw).unwrap()) // serialize to `PassportField`
                    .collect::<Passport>()
            })
            .collect())
    }
}

impl Solvable for Puzzle {
    type Solution1 = usize;

    fn part_one(input: &Self::Input) -> Option<Self::Solution1> {
        Some(
            input
                .iter()
                .filter(|passport| {
                    passport
                        .iter()
                        .filter(|field| match field {
                            PassportField::CountryId(_) => true,
                            _ => false,
                        })
                        .next()
                        .is_some()
                })
                .count(),
        )
    }
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
