pub struct Solution;

impl Solution {
    pub fn k_inverse_pairs(n: i32, k: i32) -> i32 {
        const LARGE: i32 = 1_000_000_007;
        let max_k = n * (n - 1) / 2;
        if k > max_k {
            return 0;
        }
        let k = k.min(max_k - k) as usize;
        let mut dp = vec![0; k + 1];
        let mut dp2 = vec![0; k + 1];
        dp[0] = 1;
        for i in 2..=n as usize {
            let mut sum = 0;
            for j in 0..k + 1 {
                sum = (sum + dp[j]) % LARGE;
                if j >= i {
                    if dp[j - i] > sum {
                        sum += LARGE;
                    }
                    sum -= dp[j - i];
                }
                dp2[j] = sum;
            }
            std::mem::swap(&mut dp, &mut dp2);
        }
        dp[k]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testcase_1() {
        let n = 3;
        let k = 0;
        let result = 1;
        assert_eq!(Solution::k_inverse_pairs(n, k), result);
    }

    #[test]
    fn testcase_2() {
        let n = 3;
        let k = 1;
        let result = 2;
        assert_eq!(Solution::k_inverse_pairs(n, k), result);
    }
}
