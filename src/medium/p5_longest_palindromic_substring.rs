struct Solution;

impl Solution {
    fn longest_palindrome(s: String) -> String {
        let mut len = s.len();
        let mut palindromes = vec![vec![None::<bool>; s.len()]; s.len()];
        for i in 0..s.len() {
            for start in 0..=i {
                if Self::is_palindrome(&s, start, len, &mut palindromes) {
                    return s[start..start + len].to_string();
                }
            }
            len -= 1;
        }

        s[0..1].to_string()
    }

    fn is_palindrome(
        s: &str,
        start: usize,
        len: usize,
        palindromes: &mut [Vec<Option<bool>>],
    ) -> bool {
        if let Some(res) = palindromes[start][len - 1] {
            return res;
        }

        let res = if len == 1 {
            true
        } else if s[start..start + 1] == s[start + len - 1..start + len] {
            if len == 2 {
                true
            } else {
                Self::is_palindrome(s, start + 1, len - 2, palindromes)
            }
        } else {
            false
        };

        palindromes[start][len - 1] = Some(res);
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let s = "babad".to_string();
        let result = Solution::longest_palindrome(s);
        assert!(result == "bab" || result == "aba");
    }

    #[test]
    fn test_case_2() {
        let s = "cbbd".to_string();
        let result = Solution::longest_palindrome(s);
        assert_eq!(result, "bb");
    }
}
