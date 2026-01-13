struct Solution;

impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let mut res = String::new();

        let symbols = [
            (3000, "MMM"),
            (2000, "MM"),
            (1000, "M"),
            (900, "CM"),
            (800, "DCCC"),
            (700, "DCC"),
            (600, "DC"),
            (500, "D"),
            (400, "CD"),
            (300, "CCC"),
            (200, "CC"),
            (100, "C"),
            (90, "XC"),
            (80, "LXXX"),
            (70, "LXX"),
            (60, "LX"),
            (50, "L"),
            (40, "XL"),
            (30, "XXX"),
            (20, "XX"),
            (10, "X"),
            (9, "IX"),
            (8, "VIII"),
            (7, "VII"),
            (6, "VI"),
            (5, "V"),
            (4, "IV"),
            (3, "III"),
            (2, "II"),
            (1, "I"),
        ];

        for power in (0..=3).rev() {
            let ten_to_power = 10i32.pow(power);
            let v = num / ten_to_power % 10;
            if v != 0 {
                res += symbols
                    .iter()
                    .find(|(v1, _)| *v1 == v * ten_to_power)
                    .unwrap()
                    .1;
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
        assert_eq!(Solution::int_to_roman(3749), "MMMDCCXLIX")
    }

    #[test]
    fn test_case_2() {
        assert_eq!(Solution::int_to_roman(58), "LVIII")
    }

    #[test]
    fn test_case_3() {
        assert_eq!(Solution::int_to_roman(1994), "MCMXCIV")
    }
}
