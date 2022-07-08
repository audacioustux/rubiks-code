pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn min_cost(houses: Vec<i32>, cost: Vec<Vec<i32>>, m: i32, n: i32, target: i32) -> i32 {
        let m = m as usize;
        let target = target as usize;

        (1..=n)
            .filter_map(|i| Self::dp(m, i, target, &mut HashMap::new(), &houses, &cost, n))
            .min()
            .unwrap_or(-1)
    }

    fn dp(
        size: usize,
        color: i32,
        target: usize,
        memo: &mut HashMap<(usize, i32, usize), Option<i32>>,
        houses: &[i32],
        cost: &[Vec<i32>],
        n: i32,
    ) -> Option<i32> {
        if target < 1 {
            return None;
        }
        if size < target {
            return None;
        }
        if houses[size - 1] != 0 && color != houses[size - 1] {
            return None;
        }
        if size == 1 {
            if target == 1 {
                if houses[0] == 0 {
                    Some(cost[0][(color - 1) as usize])
                } else {
                    if houses[0] == color {
                        Some(0)
                    } else {
                        None
                    }
                }
            } else {
                None
            }
        } else {
            if let Some(&res) = memo.get(&(size, color, target)) {
                return res;
            }

            let mut min = i32::MAX;
            let x = if houses[size - 1] == 0 {
                cost[size - 1][(color - 1) as usize]
            } else {
                0
            };

            for i in 1..=n {
                if let Some(y) = Self::dp(
                    size - 1,
                    i,
                    if i == color { target } else { target - 1 },
                    memo,
                    houses,
                    cost,
                    n,
                ) {
                    min = min.min(x + y);
                }
            }
            let res = if min == i32::MAX { None } else { Some(min) };

            memo.insert((size, color, target), res);

            res
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testcase_1() {
        assert_eq!(
            Solution::min_cost(
                vec![0, 0, 0, 0, 0],
                vec![
                    vec![1, 10],
                    vec![10, 1],
                    vec![10, 1],
                    vec![1, 10],
                    vec![5, 1]
                ],
                5,
                2,
                3
            ),
            9
        )
    }

    #[test]
    fn testcase_2() {
        assert_eq!(
            Solution::min_cost(
                vec![0, 2, 1, 2, 0],
                vec![
                    vec![1, 10],
                    vec![10, 1],
                    vec![10, 1],
                    vec![1, 10],
                    vec![5, 1]
                ],
                5,
                2,
                3
            ),
            11
        )
    }

    #[test]
    fn testcase_3() {
        assert_eq!(
            Solution::min_cost(
                vec![0, 0, 0, 0, 0],
                vec![
                    vec![1, 10],
                    vec![10, 1],
                    vec![1, 10],
                    vec![10, 1],
                    vec![1, 10]
                ],
                5,
                2,
                5
            ),
            5
        )
    }

    #[test]
    fn testcase_4() {
        assert_eq!(
            Solution::min_cost(
                vec![3, 1, 2, 3],
                vec![vec![1, 1, 1], vec![1, 1, 1], vec![1, 1, 1], vec![1, 1, 1],],
                4,
                3,
                3
            ),
            -1
        )
    }
}
