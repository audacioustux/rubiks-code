pub struct Solution;

impl Solution {
    pub fn max_area_of_island(mut grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();

        fn flood_fill(image: &mut Vec<Vec<i32>>, sr: usize, sc: usize) -> i32 {
            let mut stack = vec![(sr, sc)];
            let m = image.len();
            let n = image[0].len();

            let mut count = 0;
            while let Some((r, c)) = stack.pop() {
                if image[r][c] == 1 {
                    image[r][c] = 2;
                    count += 1;
                    if r > 0 {
                        stack.push((r - 1, c));
                    }
                    if c > 0 {
                        stack.push((r, c - 1));
                    }
                    if r < m - 1 {
                        stack.push((r + 1, c));
                    }
                    if c < n - 1 {
                        stack.push((r, c + 1));
                    }
                }
            }
            count
        }

        let mut max_area = 0;
        for r in 0..m {
            for c in 0..n {
                if grid[r][c] == 1 {
                    max_area = max_area.max(flood_fill(&mut grid, r, c));
                }
            }
        }
        max_area
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testcase_1() {
        let grid = vec![
            vec![0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
            vec![0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0],
            vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0],
        ];
        assert_eq!(Solution::max_area_of_island(grid), 6);
    }

    #[test]
    fn testcase_2() {
        let grid = vec![vec![0, 0, 0, 0, 0, 0, 0, 0]];
        assert_eq!(Solution::max_area_of_island(grid), 0);
    }
}
