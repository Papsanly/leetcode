struct Solution;

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let n = matrix.len();
        let m = matrix[0].len();
        let mut col = m - 1;
        let mut row = 0;
        loop {
            match matrix[row][col].cmp(&target) {
                std::cmp::Ordering::Less => {
                    if row == n - 1 {
                        return false;
                    } else {
                        row += 1;
                    }
                }
                std::cmp::Ordering::Equal => return true,
                std::cmp::Ordering::Greater => {
                    if col == 0 {
                        return false;
                    } else {
                        col -= 1;
                    }
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert!(Solution::search_matrix(
            vec![
                vec![1, 4, 7, 11, 15],
                vec![2, 5, 8, 12, 19],
                vec![3, 6, 9, 16, 22],
                vec![10, 13, 14, 17, 24],
                vec![18, 21, 23, 26, 30]
            ],
            5
        ));
    }

    #[test]
    fn test_case_2() {
        assert!(!Solution::search_matrix(
            vec![
                vec![1, 4, 7, 11, 15],
                vec![2, 5, 8, 12, 19],
                vec![3, 6, 9, 16, 22],
                vec![10, 13, 14, 17, 24],
                vec![18, 21, 23, 26, 30]
            ],
            20
        ));
    }

    #[test]
    fn test_case_3() {
        assert!(Solution::search_matrix(vec![vec![-5]], -5));
    }
}
