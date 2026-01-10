struct Solution;

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut res = 0;
        let mut negative = false;
        let mut first_pass = true;
        for byte in s.bytes() {
            if byte == b'-' && first_pass {
                first_pass = false;
                negative = true;
            } else if byte == b' ' && first_pass {
                continue;
            } else if byte == b'+' && first_pass {
                first_pass = false;
                continue;
            } else if !byte.is_ascii_digit() {
                break;
            } else {
                let digit = (byte - b'0') as i32;
                res =
                    10i32
                        .saturating_mul(res)
                        .saturating_add(if negative { -digit } else { digit });
            }
            first_pass = false;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(Solution::my_atoi("42".to_string()), 42)
    }

    #[test]
    fn test_case_2() {
        assert_eq!(Solution::my_atoi("   -042".to_string()), -42)
    }

    #[test]
    fn test_case_3() {
        assert_eq!(Solution::my_atoi("1337c0d3".to_string()), 1337)
    }

    #[test]
    fn test_case_4() {
        assert_eq!(Solution::my_atoi("0-1".to_string()), 0)
    }

    #[test]
    fn test_case_5() {
        assert_eq!(Solution::my_atoi("words and 987".to_string()), 0)
    }

    #[test]
    fn test_case_6() {
        assert_eq!(Solution::my_atoi("+-12".to_string()), 0)
    }
}
