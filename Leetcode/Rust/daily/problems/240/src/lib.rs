pub struct Solution;

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let (mut i, mut j) = (0, matrix[0].len() - 1);
        while i < matrix.len() {
            match matrix[i][j].cmp(&target) {
                std::cmp::Ordering::Less => i += 1,
                std::cmp::Ordering::Equal => return true,
                std::cmp::Ordering::Greater if j > 0 => j -= 1,
                std::cmp::Ordering::Greater => break,
            }
        }
        false
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testcase_1() {
        let matrix = vec![
            vec![1, 4, 7, 11, 15],
            vec![2, 5, 8, 12, 19],
            vec![3, 6, 9, 16, 22],
            vec![10, 13, 14, 17, 24],
            vec![18, 21, 23, 26, 30],
        ];
        let target = 5;
        assert_eq!(Solution::search_matrix(matrix, target), true);
    }

    #[test]
    fn testcase_2() {
        let matrix = vec![
            vec![1, 4, 7, 11, 15],
            vec![2, 5, 8, 12, 19],
            vec![3, 6, 9, 16, 22],
            vec![10, 13, 14, 17, 24],
            vec![18, 21, 23, 26, 30],
        ];
        let target = 20;
        assert_eq!(Solution::search_matrix(matrix, target), false);
    }
}
