struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let s = s.into_bytes();

        let mut start = 0;
        let mut end = 0;
        let mut max = 0;

        let mut map = [-1; 128];

        while end < s.len() as i16 {
            let i = s[end as usize] as usize;
            let p = map[i];
            if p >= start {
                start = p + 1;
            }

            map[i] = end;
            end += 1;

            if end - start > max {
                max = end - start
            }
        }

        max as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            Solution::length_of_longest_substring("abcabcbb".to_string()),
            3
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            Solution::length_of_longest_substring("bbbbb".to_string()),
            1
        );
    }

    #[test]
    fn test_case_3() {
        assert_eq!(
            Solution::length_of_longest_substring("pwwkew".to_string()),
            3
        );
    }

    #[test]
    fn test_case_4() {
        assert_eq!(Solution::length_of_longest_substring(" ".to_string()), 1);
    }
}
