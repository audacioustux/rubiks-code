use std::{iter::once, mem::replace};

pub struct Solution;

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let num_rows = num_rows as usize;

        fn compute_next_row(prev_row: &Vec<i32>) -> Vec<i32> {
            let middle = prev_row.windows(2).map(|w| w.iter().sum());
            once(1).chain(middle).chain(once(1)).collect()
        }

        let rows = (0..).scan(vec![1], |state, _| {
            Some(replace(state, compute_next_row(state)))
        });

        rows.take(num_rows).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testcase_1() {
        let num_rows = 5;
        let result = vec![
            vec![1],
            vec![1, 1],
            vec![1, 2, 1],
            vec![1, 3, 3, 1],
            vec![1, 4, 6, 4, 1],
        ];
        assert_eq!(Solution::generate(num_rows), result);
    }

    #[test]
    fn testcase_2() {
        let num_rows = 1;
        let result = vec![vec![1]];
        assert_eq!(Solution::generate(num_rows), result);
    }
}
