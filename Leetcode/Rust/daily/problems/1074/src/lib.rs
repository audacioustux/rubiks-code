pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn num_submatrix_sum_target(matrix: Vec<Vec<i32>>, target: i32) -> i32 {
        let (m, n) = (matrix.len(), matrix[0].len());
        let mut cum_sums = vec![vec![0; n + 1]; m + 1];

        // Create matrix with cumulative sums of matrix
        for i in 1..=m {
            let mut row_sum = 0;
            for j in 1..=n {
                row_sum += matrix[i - 1][j - 1];
                cum_sums[i][j] = cum_sums[i - 1][j] + row_sum;
            }
        }

        let mut map = HashMap::<i32, i32>::new();
        let mut rez = 0;

        // Iterate over pairs of rows (beginning to end of row span)
        for r2 in 1..=m {
            for r1 in 0..r2 {
                // Hash map is built for every row span
                map.clear();
                for c in 0..=n {
                    // sum of (r1..=r2, 0..=c) submatrix
                    let sum = cum_sums[r2][c] - cum_sums[r1][c];
                    // Add the count of submatrices in this row span that has
                    // a sum that is the difference between sum and the target
                    // to the result.
                    rez += map.get(&(sum - target)).unwrap_or(&0);
                    // Increase the count of submatrices with this sum
                    *map.entry(sum).or_insert(0) += 1;
                }
            }
        }

        rez
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testcase_1() {
        let matrix = vec![vec![0, 1, 0], vec![1, 1, 1], vec![0, 1, 0]];
        let target = 0;
        let result = 4;
        assert_eq!(Solution::num_submatrix_sum_target(matrix, target), result);
    }

    #[test]
    fn testcase_2() {
        let matrix = vec![vec![1, -1], vec![-1, 1]];
        let target = 0;
        let result = 5;
        assert_eq!(Solution::num_submatrix_sum_target(matrix, target), result);
    }

    #[test]
    fn testcase_3() {
        let matrix = vec![vec![904]];
        let target = 0;
        let result = 0;
        assert_eq!(Solution::num_submatrix_sum_target(matrix, target), result);
    }
}
