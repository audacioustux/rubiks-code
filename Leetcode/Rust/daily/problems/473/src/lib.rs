pub struct Solution;

impl Solution {
    pub fn makesquare(matchsticks: Vec<i32>) -> bool {
        let mut matchsticks = matchsticks;
        matchsticks.sort_unstable();
        matchsticks.reverse();

        let sum: i32 = matchsticks.iter().sum();
        let mut square_sides = vec![sum / 4; 4];

        fn eval(matchsticks: &Vec<i32>, square_sides: &mut Vec<i32>, match_index: usize) -> bool {
            if match_index == matchsticks.len() {
                for side in square_sides {
                    if *side != 0 {
                        return false;
                    }
                }
                return true;
            }

            for square_side_index in 0..4 {
                if matchsticks[match_index] > square_sides[square_side_index] {
                    continue;
                }

                square_sides[square_side_index] -= matchsticks[match_index];
                if eval(matchsticks, square_sides, match_index + 1) {
                    return true;
                }
                square_sides[square_side_index] += matchsticks[match_index];
            }

            false
        }

        sum % 4 == 0 && eval(&matchsticks, &mut square_sides, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testcase_1() {
        let matchsticks = vec![1, 1, 2, 2, 2];
        assert_eq!(Solution::makesquare(matchsticks), true);
    }

    #[test]
    fn testcase_2() {
        let matchsticks = vec![3, 3, 3, 3, 4];
        assert_eq!(Solution::makesquare(matchsticks), false);
    }

    #[test]
    fn testcase_3() {
        let matchsticks = vec![3, 3, 3, 1, 2, 2, 2, 3, 1, 1, 1, 2];
        assert_eq!(Solution::makesquare(matchsticks), true);
    }

    #[test]
    fn testcase_4() {
        let matchsticks = vec![12, 8, 12, 16, 20, 24, 28, 32, 36, 40, 44, 48, 52, 56, 60];
        assert_eq!(Solution::makesquare(matchsticks), false);
    }
}
