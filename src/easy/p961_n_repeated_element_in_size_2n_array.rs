use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn repeated_n_times(nums: Vec<i32>) -> i32 {
        let mut set = HashSet::with_capacity(nums.len());
        for num in nums {
            if set.contains(&num) {
                return num;
            } else {
                set.insert(num);
            }
        }

        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let nums = vec![1, 2, 3, 3];
        let result = Solution::repeated_n_times(nums);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_case_2() {
        let nums = vec![5, 1, 5, 2, 5, 3, 5, 4];
        let result = Solution::repeated_n_times(nums);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_case_3() {
        let nums = vec![2, 1, 2, 5, 3, 2];
        let result = Solution::repeated_n_times(nums);
        assert_eq!(result, 2);
    }
}
