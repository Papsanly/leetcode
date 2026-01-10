use std::{cmp, collections::HashSet};

struct Solution;

impl Solution {
    pub fn string_matching(mut words: Vec<String>) -> Vec<String> {
        let mut result = HashSet::new();
        words.sort_by_key(|word| cmp::Reverse(word.len()));
        for (i, word) in words.iter().enumerate() {
            for another_word in words[i + 1..].iter() {
                if word.contains(another_word) {
                    result.insert(another_word.clone());
                }
            }
        }
        result.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            Solution::string_matching(
                ["mass", "as", "hero", "superhero"]
                    .into_iter()
                    .map(|s| s.to_string())
                    .collect()
            )
            .into_iter()
            .collect::<HashSet<_>>(),
            ["as", "hero"]
                .into_iter()
                .map(|s| s.to_string())
                .collect::<HashSet<_>>()
        )
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            Solution::string_matching(
                ["leetcode", "et", "code"]
                    .into_iter()
                    .map(|s| s.to_string())
                    .collect()
            )
            .into_iter()
            .collect::<HashSet<_>>(),
            ["et", "code"]
                .into_iter()
                .map(|s| s.to_string())
                .collect::<HashSet<_>>()
        )
    }

    #[test]
    fn test_case_3() {
        assert_eq!(
            Solution::string_matching(
                ["blue", "green", "bu"]
                    .iter()
                    .map(|s| s.to_string())
                    .collect()
            )
            .into_iter()
            .collect::<HashSet<_>>(),
            HashSet::new()
        )
    }
}
