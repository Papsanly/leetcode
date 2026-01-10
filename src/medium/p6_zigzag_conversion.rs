struct Solution;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let num_rows = num_rows as usize;
        let bytes = s.into_bytes();
        let mut res = Vec::with_capacity(bytes.len());

        if num_rows == 1 {
            return unsafe { String::from_utf8_unchecked(bytes) };
        }

        for j in 0..num_rows {
            for i in 0..=bytes.len().div_ceil(2 * (num_rows - 1)) {
                let a = (num_rows - 1) * 2 * i;
                let i1 = a.checked_sub(j);
                if let Some(i1) = i1
                    && j != 0
                    && j != num_rows - 1
                    && i1 < bytes.len()
                {
                    res.push(bytes[i1]);
                }
                let i2 = a + j;
                if i2 < bytes.len() {
                    res.push(bytes[i2]);
                }
            }
        }

        unsafe { String::from_utf8_unchecked(res) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let s = "PAYPALISHIRING".to_string();
        let num_rows = 3;
        let result = Solution::convert(s, num_rows);
        assert_eq!(result, "PAHNAPLSIIGYIR");
    }

    #[test]
    fn test_case_2() {
        let s = "PAYPALISHIRING".to_string();
        let num_rows = 4;
        let result = Solution::convert(s, num_rows);
        assert_eq!(result, "PINALSIGYAHRPI");
    }

    #[test]
    fn test_case_3() {
        let s = "A".to_string();
        let num_rows = 1;
        let result = Solution::convert(s, num_rows);
        assert_eq!(result, "A");
    }
}
