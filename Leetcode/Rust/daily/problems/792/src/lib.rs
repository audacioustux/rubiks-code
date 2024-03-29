use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn num_matching_subseq(s: String, words: Vec<String>) -> i32 {
        let mut map: HashMap<String, i32> = HashMap::new();
        for word in words.iter() {
            let entry = map.entry(word.to_string()).or_insert(0);
            *entry += 1;
        }
        let mut ans: i32 = 0;
        for (word, c) in map {
            let (mut i, mut j) = (0, 0);
            while i < word.len() && j < s.len() {
                if word[i..i + 1] == s[j..j + 1] {
                    i += 1;
                }
                j += 1;
            }
            if i == word.len() {
                ans += c;
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testcase_1() {
        let s = "abcde".to_owned();
        let words = ["a", "bb", "acd", "ace"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let result = 3;
        assert_eq!(Solution::num_matching_subseq(s, words), result);
    }
}
