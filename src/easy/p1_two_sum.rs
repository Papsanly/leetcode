use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();

        for (i, num) in nums.iter().enumerate() {
            if let Some(&j) = map.get(&(target - num)) {
                return vec![j, i as i32];
            } else {
                map.insert(num, i as i32);
            }
        }

        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1])
    }

    #[test]
    fn test_case_2() {
        assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2])
    }

    #[test]
    fn test_case_3() {
        assert_eq!(Solution::two_sum(vec![3, 3], 6), vec![0, 1])
    }
}
