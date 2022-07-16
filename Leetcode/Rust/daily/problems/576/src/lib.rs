pub struct Solution;

impl Solution {
    pub fn find_paths(m: i32, n: i32, max_move: i32, start_row: i32, start_column: i32) -> i32 {
        let mut paths_oob = 0;
        let mut dp = vec![vec![0; n as usize]; m as usize];
        let mut new_dp = vec![vec![0; n as usize]; m as usize];
        dp[start_row as usize][start_column as usize] = 1;
        for _ in 0..max_move {
            for dp_row in new_dp.iter_mut() {
                dp_row.fill(0);
            }
            for x in 0..m {
                for y in 0..n {
                    let cur_dp = dp[x as usize][y as usize];
                    if cur_dp == 0 {
                        continue;
                    }
                    for (adj_x, adj_y) in [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)] {
                        if adj_x < 0 || m as i32 <= adj_x || adj_y < 0 || n <= adj_y {
                            paths_oob += cur_dp;
                            paths_oob %= 1e9 as i32 + 7;
                        } else {
                            new_dp[adj_x as usize][adj_y as usize] += cur_dp;
                            new_dp[adj_x as usize][adj_y as usize] %= 1e9 as i32 + 7;
                        }
                    }
                }
            }
            for x in 0..m as usize {
                for y in 0..n as usize {
                    dp[x][y] = new_dp[x][y];
                }
            }
        }
        paths_oob
    }
}

#[cfg(test)]
mod tests {}
