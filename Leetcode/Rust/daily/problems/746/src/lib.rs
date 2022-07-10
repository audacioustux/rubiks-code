pub struct Solution;

impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let (_, min_cost) = cost
            .iter()
            .chain(&[0])
            .fold((0, 0), |(x, y), t| (y, t + x.min(y)));

        min_cost
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testcase_1() {
        let cost = vec![10, 15, 20];
        assert_eq!(Solution::min_cost_climbing_stairs(cost), 15);
    }

    #[test]
    fn testcase_2() {
        let cost = vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1];
        assert_eq!(Solution::min_cost_climbing_stairs(cost), 6);
    }
}
