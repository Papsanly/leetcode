struct Solution;

impl Solution {
    pub fn min_changes(s: String) -> i32 {
        let s = s.as_bytes();
        (0..s.len())
            .step_by(2)
            .filter(|&i| s[i] != s[i + 1])
            .count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(Solution::min_changes("1001".to_string()), 2)
    }

    #[test]
    fn test_case_2() {
        assert_eq!(Solution::min_changes("10".to_string()), 1)
    }

    #[test]
    fn test_case_3() {
        assert_eq!(Solution::min_changes("0000".to_string()), 0)
    }
}
