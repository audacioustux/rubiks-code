pub struct Solution;

impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        if s1.len() + s2.len() != s3.len() {
            return false;
        }

        let mut s1 = s1.chars().peekable();
        let mut s2 = s2.chars().peekable();
        let mut s3 = s3.chars().peekable();

        while let Some(c3) = s3.peek() {
            if s1.next_if_eq(&c3).is_some() {
                s3.next();
                continue;
            }
            if s2.next_if_eq(&c3).is_some() {
                s3.next();
                let tmp = s1;
                s1 = s2;
                s2 = tmp;
                // (s1, s2) = (s2, s1);
                continue;
            }
            return false;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testcase_1() {
        let s1 = "aabcc".to_string();
        let s2 = "dbbca".to_string();
        let s3 = "aadbbcbcac".to_string();
        let result = Solution::is_interleave(s1, s2, s3);
        assert_eq!(result, true);
    }

    #[test]
    fn testcase_2() {
        let s1 = "aabcc".to_string();
        let s2 = "dbbcc".to_string();
        let s3 = "aadbbcbcac".to_string();
        let result = Solution::is_interleave(s1, s2, s3);
        assert_eq!(result, false);
    }

    #[test]
    fn testcase_3() {
        let s1 = "aabcc".to_string();
        let s2 = "dbbca".to_string();
        let s3 = "aadbbbaccc".to_string();
        let result = Solution::is_interleave(s1, s2, s3);
        assert_eq!(result, false);
    }

    #[test]
    fn testcase_4() {
        let s1 = "".to_string();
        let s2 = "".to_string();
        let s3 = "".to_string();
        let result = Solution::is_interleave(s1, s2, s3);
        assert_eq!(result, true);
    }
}
