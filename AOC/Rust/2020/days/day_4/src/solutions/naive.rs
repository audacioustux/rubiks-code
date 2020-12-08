use aoclib::*;
use itertools::{join, Itertools};
use lazy_static::lazy_static;
use regex::Regex;
use std::default::Default;
use std::ops::RangeInclusive;

pub struct Puzzle;

impl InputParsable for Puzzle {
    type Input = Vec<Passport>;

    fn parse_input(raw_input: &str) -> Result<Self::Input> {
        Ok(raw_input
            .trim() // santize input
            .lines()
            .map(|line| line.trim().to_string()) // sanitize incidents
            .group_by(|line| line.is_empty()) // group passport data, seperated by blank line
            .into_iter()
            .map(|(_, lines)| join(lines, " ")) // join all fields in one line
            .filter(|lines| !lines.is_empty()) // ignore empty lines
            .filter_map(|passport_lines| Passport::new(&passport_lines))
            .map(|passport| passport.validate())
            .collect())
    }
}

#[derive(Debug)]
enum EyeColor {
    Amb,
    Blu,
    Brn,
    Gry,
    Grn,
    Hzl,
    Oth,
}

#[derive(Debug)]
enum Height {
    Centimeter(u32),
    Inch(u32),
}

#[derive(Debug)]
enum PassportField<T> {
    Valid(T),
    Some(String),
    None,
}

impl<T> Default for PassportField<T> {
    fn default() -> Self {
        Self::None
    }
}

impl<T> PassportField<T> {
    fn is_some(self: &Self) -> bool {
        match self {
            Self::Some(_) => true,
            Self::Valid(_) => true,
            _ => false,
        }
    }
    fn is_some_valid(self: &Self) -> bool {
        match self {
            Self::Valid(_) => true,
            _ => false,
        }
    }
    fn to_valid<F>(self: Self, validator: F) -> Self
    where
        F: Fn(&String) -> Option<T>,
    {
        if let Self::Some(s) = &self {
            if let Some(v) = validator(s) {
                return Self::Valid(v);
            }
        }
        self
    }
}

#[derive(Debug, Default)]
pub struct Passport {
    birth_year: PassportField<u32>,
    issue_year: PassportField<u32>,
    expiration_year: PassportField<u32>,
    height: PassportField<Height>,
    hair_color: PassportField<String>,
    eye_color: PassportField<EyeColor>,
    passport_id: PassportField<u32>,
    country_id: PassportField<String>,
}

impl Passport {
    fn has_required_fields(self: &Self) -> bool {
        self.birth_year.is_some()
            && self.issue_year.is_some()
            && self.expiration_year.is_some()
            && self.height.is_some()
            && self.hair_color.is_some()
            && self.eye_color.is_some()
            && self.passport_id.is_some()
    }

    fn is_valid(self: &Self) -> bool {
        self.birth_year.is_some_valid()
            && self.issue_year.is_some_valid()
            && self.expiration_year.is_some_valid()
            && self.height.is_some_valid()
            && self.hair_color.is_some_valid()
            && self.eye_color.is_some_valid()
            && self.passport_id.is_some_valid()
    }

    fn new(input: &str) -> Option<Self> {
        lazy_static! {
            static ref FIELDS_RE: Regex = Regex::new(r"(?P<field>\w+):(?P<value>\S*)").unwrap();
        }

        let mut passport = Passport::default();

        FIELDS_RE.captures_iter(input).for_each(|m| {
            use PassportField::*;

            let value = m["value"].to_string();
            match &m["field"] {
                "byr" => passport.birth_year = Some(value),
                "iyr" => passport.issue_year = Some(value),
                "eyr" => passport.expiration_year = Some(value),
                "hgt" => passport.height = Some(value),
                "hcl" => passport.hair_color = Some(value),
                "ecl" => passport.eye_color = Some(value),
                "pid" => passport.passport_id = Some(value),
                "cid" => passport.country_id = Some(value),
                _ => (),
            };
        });

        Some(passport)
    }

    fn validate(self: Self) -> Self {
        // NOTE: https://github.com/rust-lang/rust/pull/79051
        // sed :(

        let Passport {
            birth_year,
            issue_year,
            expiration_year,
            height,
            hair_color,
            eye_color,
            passport_id,
            country_id,
        } = self;

        let parse_u32 = |s: &String| s.parse::<u32>().ok();
        let in_range = |n: u32, range: RangeInclusive<u32>| range.contains(&n).then_some(n);

        let birth_year =
            birth_year.to_valid(|s| parse_u32(s).and_then(|n| in_range(n, 1920..=2002)));

        let issue_year =
            issue_year.to_valid(|s| parse_u32(s).and_then(|n| in_range(n, 2010..=2020)));

        let expiration_year =
            expiration_year.to_valid(|s| parse_u32(s).and_then(|n| in_range(n, 2020..=2030)));

        lazy_static! {
            static ref HGT_RE: Regex = Regex::new(r"^(\d+)(cm|in)$").unwrap();
        }
        let height = height.to_valid(|s| {
            if let Some(m) = HGT_RE.captures(&s) {
                let (value, unit) = (&m[1], &m[2]);
                return match (unit, parse_u32(&value.to_string())) {
                    ("cm", Some(v)) => {
                        if (150..=193).contains(&v) {
                            Some(Height::Centimeter(v))
                        } else {
                            None
                        }
                    }
                    ("in", Some(v)) => {
                        if (59..=76).contains(&v) {
                            Some(Height::Inch(v))
                        } else {
                            None
                        }
                    }
                    (_, _) => None,
                };
            }
            None
        });

        lazy_static! {
            static ref HCL_RE: Regex = Regex::new(r"^#([a-f0-9]{6})$").unwrap();
        }
        let hair_color = hair_color.to_valid(|s| {
            if let Some(c) = HCL_RE.captures(&s) {
                Some(c[1].to_string())
            } else {
                None
            }
        });

        let eye_color = eye_color.to_valid(|color| {
            use EyeColor::*;
            match color.as_str() {
                "amb" => Some(Amb),
                "blu" => Some(Blu),
                "brn" => Some(Brn),
                "gry" => Some(Gry),
                "grn" => Some(Grn),
                "hzl" => Some(Hzl),
                "oth" => Some(Oth),
                _ => None,
            }
        });

        let passport_id = passport_id.to_valid(|s| if s.len() == 9 { parse_u32(s) } else { None });

        Self {
            birth_year,
            issue_year,
            expiration_year,
            height,
            hair_color,
            eye_color,
            passport_id,
            country_id,
        }
    }
}

impl Solvable for Puzzle {
    fn part_one(input: &Self::Input) -> Option<Self::Solution1> {
        Some(
            input
                .iter()
                .filter(|passport| passport.has_required_fields())
                .count() as u32,
        )
    }
    fn part_two(input: &Self::Input) -> Option<Self::Solution2> {
        Some(input.iter().filter(|passport| passport.is_valid()).count() as u32)
    }
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
