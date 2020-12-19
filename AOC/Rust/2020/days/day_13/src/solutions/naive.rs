use aoclib::*;

pub struct Puzzle;

impl InputParsable for Puzzle {
    type Input = (u64, Vec<String>);

    fn parse_input(raw_input: &str) -> Result<Self::Input> {
        let mut input = raw_input.trim().lines();
        let timestamp = input.next().unwrap().parse().unwrap();
        let bus_ids = input
            .next()
            .unwrap()
            .trim()
            .split(',')
            .map(|bus_id| bus_id.to_string())
            .collect();

        Ok((timestamp, bus_ids))
    }
}
impl Solvable for Puzzle {
    type Solution1 = u64;
    type Solution2 = i64;

    fn part_one(input: &Self::Input) -> Option<Self::Solution1> {
        input
            .1
            .iter()
            .filter(|id| *id != "x")
            .map(|id| id.parse().unwrap())
            .map(|id: u64| (id, (input.0 as i64).wrapping_neg().rem_euclid(id as i64)))
            .min_by(|x, y| x.1.cmp(&y.1))
            .map(|(b_id, wait_t)| b_id * wait_t as u64)
    }
    fn part_two(input: &Self::Input) -> Option<Self::Solution2> {
        let bus_id_iter: (Vec<i64>, Vec<i64>) = input
            .1
            .iter()
            .enumerate()
            .filter(|(_, id)| *id != "x")
            .map(|(i, id)| (i, id.parse::<i64>().unwrap()))
            .map(|(i, id)| (id - i as i64, id))
            .unzip();

        Self::chinese_remainder(&bus_id_iter.0, &bus_id_iter.1)
    }
}

impl Puzzle {
    fn chinese_remainder(residues: &[i64], modulii: &[i64]) -> Option<i64> {
        let prod = modulii.iter().product::<i64>();
        let mut sum = 0;

        for (&residue, &modulus) in residues.iter().zip(modulii) {
            let p = prod / modulus;
            sum += residue * p * Self::mod_inv(p, modulus)?;
        }

        Some(sum % prod)
    }

    fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
        if a == 0 {
            (b, 0, 1)
        } else {
            let (g, x, y) = Self::egcd(b % a, a);
            (g, y - (b / a) * x, x)
        }
    }

    fn mod_inv(x: i64, n: i64) -> Option<i64> {
        let (g, x, _) = Self::egcd(x, n);

        if g == 1 {
            Some((x % n + n) % n)
        } else {
            None
        }
    }
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
