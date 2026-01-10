struct Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        if x == i32::MIN {
            return 0;
        }

        let mut y = if x < 0 { -x } else { x };
        let mut res = 0i32;

        let mut total_digits = 1;
        while total_digits < 10 && 10i32.pow(total_digits) <= y {
            total_digits += 1;
        }

        for power in (0..total_digits).rev() {
            let digit = y % 10;
            let Some(mul_res) = digit.checked_mul(10i32.pow(power)) else {
                return 0;
            };
            let Some(add_res) = res.checked_add(mul_res) else {
                return 0;
            };
            res = add_res;
            y /= 10;
        }

        if x < 0 { -res } else { res }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(Solution::reverse(123), 321);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(Solution::reverse(-123), -321);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(Solution::reverse(120), 21);
    }
}
