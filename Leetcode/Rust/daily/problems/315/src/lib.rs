use std::collections::HashSet;
use std::convert::TryInto;

pub struct Solution;

impl Solution {
    pub fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
        let mut numbers_left = {
            let unique_numbers: HashSet<_> = nums.iter().copied().collect();
            let mut num_counter: Vec<(i32, u32)> = unique_numbers.iter().map(|&x| (x, 0)).collect();
            num_counter.sort_unstable();
            num_counter
        };

        let mut res = vec![0i32; nums.len()];
        for (i, &v) in nums.iter().enumerate().rev() {
            let position_number = numbers_left.binary_search_by_key(&v, |&(k, _)| k).unwrap();
            let res_count = if position_number <= nums.len() - i {
                numbers_left[..position_number]
                    .iter()
                    .map(|(_, c)| c)
                    .sum::<u32>()
            } else {
                nums[i + 1..]
                    .iter()
                    .filter(|&&x| x < v)
                    .count()
                    .try_into()
                    .unwrap()
            };
            numbers_left[position_number].1 += 1;
            res[i] = res_count.try_into().unwrap();
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let nums = vec![5, 2, 6, 1];
        let result = vec![2, 1, 1, 0];
        assert_eq!(Solution::count_smaller(nums), result);
    }
}
