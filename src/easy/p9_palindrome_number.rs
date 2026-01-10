struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        let mut rev = 0;
        let mut y = x;
        while y != 0 {
            let rem = y % 10;
            rev = 10 * rev + rem;
            y /= 10;
        }
        rev == x
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert!(Solution::is_palindrome(121))
    }

    #[test]
    fn test_case_2() {
        assert!(!Solution::is_palindrome(-121))
    }

    #[test]
    fn test_case_3() {
        assert!(!Solution::is_palindrome(10))
    }
}
