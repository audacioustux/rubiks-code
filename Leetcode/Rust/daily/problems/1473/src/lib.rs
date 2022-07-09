pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn min_cost(houses: Vec<i32>, cost: Vec<Vec<i32>>, m: i32, n: i32, target: i32) -> i32 {
        let m = m as usize;
        let target = target as usize;

        (1..=n)
            .filter_map(|color| Self::dp(m, n, color, target, &mut HashMap::new(), &houses, &cost))
            .min()
            .unwrap_or(-1)
    }

    fn dp(
        num_of_house: usize,
        num_of_color: i32,
        color: i32,
        target: usize,
        cache: &mut HashMap<(usize, i32, usize), Option<i32>>,
        houses: &[i32],
        cost: &[Vec<i32>],
    ) -> Option<i32> {
        if target < 1 {
            return None;
        }
        if num_of_house < target {
            return None;
        }
        if houses[num_of_house - 1] != 0 && color != houses[num_of_house - 1] {
            return None;
        }
        if num_of_house == 1 {
            if target == 1 {
                if houses[0] == 0 {
                    Some(cost[0][(color - 1) as usize])
                } else {
                    (houses[0] == color).then_some(0)
                }
            } else {
                None
            }
        } else {
            if let Some(&min_cost) = cache.get(&(num_of_house, color, target)) {
                return min_cost;
            }

            let mut min = i32::MAX;
            let x = if houses[num_of_house - 1] == 0 {
                cost[num_of_house - 1][(color - 1) as usize]
            } else {
                0
            };

            for i in 1..=num_of_color {
                if let Some(y) = Self::dp(
                    num_of_house - 1,
                    num_of_color,
                    i,
                    if i == color { target } else { target - 1 },
                    cache,
                    houses,
                    cost,
                ) {
                    min = min.min(x + y);
                }
            }
            let min_cost = if min == i32::MAX { None } else { Some(min) };

            cache.insert((num_of_house, color, target), min_cost);

            min_cost
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
