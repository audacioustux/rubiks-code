use aoclib::*;
use std::convert::TryFrom;

pub struct Puzzle;

//  E
// N+S
//  W
#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn right(&self, times: u64) -> Self {
        use Direction::*;
        *[North, East, South, West]
            .iter()
            .cycle()
            .skip_while(|d| *d != self)
            .nth(times as usize)
            .unwrap()
    }

    pub fn left(&self, times: u64) -> Self {
        use Direction::*;
        *[North, East, South, West]
            .iter()
            .rev()
            .cycle()
            .skip_while(|d| *d != self)
            .nth(times as usize)
            .unwrap()
    }
}

#[derive(Debug, Clone, Copy)]
struct Ship {
    east: i64,
    south: i64,
    direction: Direction,
}

impl Ship {
    fn navigate(&self, nav_ins: &NavInstruction) -> Self {
        use Direction::*;

        let Self {
            mut east,
            mut south,
            mut direction,
        } = *self;

        let NavInstruction { action, value } = *nav_ins;

        match action {
            'E' => east += value as i64,
            'W' => east -= value as i64,
            'S' => south += value as i64,
            'N' => south -= value as i64,
            'L' => direction = direction.left(value / 90),
            'R' => direction = direction.right(value / 90),
            'F' if direction == East => east += value as i64,
            'F' if direction == West => east -= value as i64,
            'F' if direction == South => south += value as i64,
            'F' if direction == North => south -= value as i64,
            _ => {}
        }

        Self {
            east,
            south,
            direction,
        }
    }
}

#[derive(Debug)]
pub struct NavInstruction {
    action: char,
    value: u64,
}
impl TryFrom<&str> for NavInstruction {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (action, value) = value.split_at(1);
        Ok(Self {
            action: action.chars().next().ok_or(())?,
            value: value.parse().map_err(|_| ())?,
        })
    }
}

impl InputParsable for Puzzle {
    type Input = Vec<NavInstruction>;

    fn parse_input(raw_input: &str) -> Result<Self::Input> {
        let input: Self::Input = raw_input
            .trim()
            .lines()
            .map(|line| line.trim())
            .map_while(|ins_raw| NavInstruction::try_from(ins_raw).ok())
            .collect();

        Ok(input)
    }
}

struct Ship2 {
    location: (i64, i64),
    waypoint: (i64, i64),
}
impl Ship2 {
    fn navigate(&self, nav_ins: &NavInstruction) -> Self {
        let Self {
            mut location,
            mut waypoint,
        } = self.clone();

        let NavInstruction { action, value } = *nav_ins;

        let rotate_fn = |waypoint: (i64, i64), value: i64| -> (i64, i64) {
            let mag = ((waypoint.0.pow(2) + waypoint.1.pow(2)) as f32).sqrt();
            let angle = (waypoint.1 as f32).atan2(waypoint.0 as f32) + (value as f32).to_radians();

            let x = (mag * angle.cos()).round();
            let y = (mag * angle.sin()).round();

            (x as i64, y as i64)
        };

        match action {
            'E' => waypoint.0 += value as i64,
            'W' => waypoint.0 -= value as i64,
            'S' => waypoint.1 -= value as i64,
            'N' => waypoint.1 += value as i64,
            'L' => waypoint = rotate_fn(waypoint, value as i64),
            'R' => waypoint = rotate_fn(waypoint, -(value as i64)),
            'F' => {
                location.0 += value as i64 * waypoint.0;
                location.1 += value as i64 * waypoint.1;
            }
            _ => {}
        }

        Self { location, waypoint }
    }
}

impl Solvable for Puzzle {
    fn part_one(input: &Self::Input) -> Option<Self::Solution1> {
        let mut ship = Ship {
            east: 0,
            south: 0,
            direction: Direction::East,
        };
        input.iter().for_each(|nav_ins| {
            ship = ship.navigate(nav_ins);
        });

        Some((ship.east.abs() + ship.south.abs()) as u32)
    }

    fn part_two(input: &Self::Input) -> Option<Self::Solution2> {
        let mut ship = Ship2 {
            location: (0, 0),
            waypoint: (10, 1),
        };
        input.iter().for_each(|nav_ins| {
            ship = ship.navigate(nav_ins);
        });

        Some((ship.location.0.abs() + ship.location.1.abs()) as u32)
    }
}

impl Puzzle {}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;

#[cfg(test)]
mod direction_tests {
    use super::*;

    #[test]
    fn north_east() {
        use Direction::*;
        assert_eq!(North.right(1), East);
    }

    #[test]
    fn west_north() {
        use Direction::*;
        assert_eq!(West.right(1), North);
    }

    #[test]
    fn east_north() {
        use Direction::*;
        assert_eq!(East.left(1), North);
    }

    #[test]
    fn north_west() {
        use Direction::*;
        assert_eq!(North.left(1), West);
    }

    #[test]
    fn south_west() {
        use Direction::*;
        assert_eq!(South.left(3), West);
    }

    #[test]
    fn south_east() {
        use Direction::*;
        assert_eq!(South.right(3), East);
    }
}
