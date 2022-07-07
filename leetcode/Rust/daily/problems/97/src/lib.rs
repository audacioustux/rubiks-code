use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        let (l1, l2, l3) = (s1.len(), s2.len(), s3.len());
        if l1 + l2 != l3 {
            return false;
        }

        let [s1, s2, s3]: [Vec<char>; 3] = [s1, s2, s3].map(|string| string.chars().collect());

        let mut cache: HashMap<(usize, usize), bool> = HashMap::new();

        cache.insert((l1, l2), true);

        for i1 in (0..=l1).rev() {
            for i2 in (0..=l2).rev() {
                if i1 < l1 && s1[i1] == s3[i1 + i2] && cache.get(&(i1 + 1, i2)).is_some() {
                    cache.insert((i1, i2), true);
                }
                if i2 < l2 && s2[i2] == s3[i1 + i2] && cache.get(&(i1, i2 + 1)).is_some() {
                    cache.insert((i1, i2), true);
                }
            }
        }

        return cache.get(&(0, 0)).is_some();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testcase_1() {
        let s1 = "aabcc".to_owned();
        let s2 = "dbbca".to_owned();
        let s3 = "aadbbcbcac".to_owned();
        let result = Solution::is_interleave(s1, s2, s3);
        assert_eq!(result, true);
    }

    #[test]
    fn testcase_2() {
        let s1 = "aabcc".to_owned();
        let s2 = "dbbcc".to_owned();
        let s3 = "aadbbcbcac".to_owned();
        let result = Solution::is_interleave(s1, s2, s3);
        assert_eq!(result, false);
    }

    #[test]
    fn testcase_3() {
        let s1 = "aabcc".to_owned();
        let s2 = "dbbca".to_owned();
        let s3 = "aadbbbaccc".to_owned();
        let result = Solution::is_interleave(s1, s2, s3);
        assert_eq!(result, false);
    }

    #[test]
    fn testcase_4() {
        let s1 = "".to_owned();
        let s2 = "".to_owned();
        let s3 = "".to_owned();
        let result = Solution::is_interleave(s1, s2, s3);
        assert_eq!(result, true);
    }

    #[test]
    fn testcase_5() {
        let s1 = "aa".to_owned();
        let s2 = "ab".to_owned();
        let s3 = "aaba".to_owned();
        let result = Solution::is_interleave(s1, s2, s3);
        assert_eq!(result, true);
    }

    #[test]
    fn testcase_6() {
        assert_eq!(
            Solution::is_interleave(
                "aabcc".to_owned(),
                "dbbca".to_owned(),
                "aadbbcbcac".to_owned()
            ),
            true
        );
    }

    #[test]
    fn testcase_7() {
        assert_eq!(
            Solution::is_interleave(
                "aabcc".to_owned(),
                "dbbca".to_owned(),
                "aadbbbaccc".to_owned()
            ),
            false
        );
    }
}
