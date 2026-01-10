use std::mem::swap;

struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> f64 {
        if nums1.len() > nums2.len() {
            swap(&mut nums1, &mut nums2);
        }

        let l1 = nums1.len();
        let l2 = nums2.len();

        let mut power = 1;
        let mut partition1 = l1 / 2usize.pow(power);

        loop {
            power += 1;

            let partition2 = l1.midpoint(l2) - partition1;

            let max_left1 = if partition1 == 0 {
                i32::MIN
            } else {
                nums1[partition1 - 1]
            };
            let min_right1 = if partition1 == l1 {
                i32::MAX
            } else {
                nums1[partition1]
            };
            let max_left2 = if partition2 == 0 {
                i32::MIN
            } else {
                nums2[partition2 - 1]
            };
            let min_right2 = if partition2 == l2 {
                i32::MAX
            } else {
                nums2[partition2]
            };

            if max_left1 <= min_right2 && max_left2 <= min_right1 {
                if (l1 + l2).is_multiple_of(2) {
                    return (max_left1.max(max_left2) as f64)
                        .midpoint(min_right1.min(min_right2) as f64);
                } else {
                    return min_right1.min(min_right2) as f64;
                }
            } else if max_left1 > min_right2 {
                partition1 -= l1 / 2usize.pow(power).min(l1);
            } else {
                partition1 += l1 / 2usize.pow(power).min(l1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 3], vec![2]),
            2.0
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]),
            2.5
        );
    }
}
