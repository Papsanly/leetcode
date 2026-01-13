struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let s = s.as_bytes();
        let mut res = 0;

        for i in 0..s.len() {
            if i < s.len() - 1 && s[i] == b'I' && (s[i + 1] == b'X' || s[i + 1] == b'V') {
                res -= 1;
            } else if i < s.len() - 1 && s[i] == b'X' && (s[i + 1] == b'L' || s[i + 1] == b'C') {
                res -= 10;
            } else if i < s.len() - 1 && s[i] == b'C' && (s[i + 1] == b'D' || s[i + 1] == b'M') {
                res -= 100;
            } else {
                res += match s[i] {
                    b'M' => 1000,
                    b'D' => 500,
                    b'C' => 100,
                    b'L' => 50,
                    b'X' => 10,
                    b'V' => 5,
                    b'I' => 1,
                    _ => unreachable!(),
                };
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(Solution::roman_to_int("III".to_string()), 3)
    }

    #[test]
    fn test_case_2() {
        assert_eq!(Solution::roman_to_int("LVIII".to_string()), 58)
    }

    #[test]
    fn test_case_3() {
        assert_eq!(Solution::roman_to_int("MCMXCIV".to_string()), 1994)
    }
}
