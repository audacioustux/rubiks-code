pub struct Solution;

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if let Ok(_) = nums.binary_search(&target) {
            vec![
                nums.partition_point(|&i| i < target) as i32,
                nums.partition_point(|&i| i <= target) as i32 - 1,
            ]
        } else {
            vec![-1, -1]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testcase_1() {
        let nums = vec![5, 7, 7, 8, 8, 10];
        let target = 8;
        let result = vec![3, 4];
        assert_eq!(Solution::search_range(nums, target), result);
    }

    #[test]
    fn testcase_2() {
        let nums = vec![5, 7, 7, 8, 8, 10];
        let target = 6;
        let result = vec![-1, -1];
        assert_eq!(Solution::search_range(nums, target), result);
    }
}
