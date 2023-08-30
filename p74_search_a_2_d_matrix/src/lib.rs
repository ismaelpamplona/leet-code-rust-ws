use std::cmp::Ordering::{Equal, Greater, Less};
struct Solution;
impl Solution {
    pub fn search_matrix_a(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let vector: Vec<i32> = matrix.iter().flat_map(|row| row.iter()).cloned().collect();
        let (mut left, mut right) = (0 as i32, vector.len() as i32 - 1);
        while left <= right {
            let mid = left + (right - left) / 2;
            if vector[mid as usize] == target {
                return true;
            } else if vector[mid as usize] > target {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }
        false
    }

    pub fn search_matrix_b(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let (m, n) = (matrix.len() as i32, matrix[0].len() as i32);
        let (mut left, mut right) = (0, m * n - 1);
        while left <= right {
            let mid = left + (right - left) / 2;
            let row = mid / n;
            let col = mid % n;
            let num = matrix[row as usize][col as usize];
            if num == target {
                return true;
            } else if num > target {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }
        false
    }

    pub fn search_matrix_c(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let (m, n) = (matrix.len(), matrix[0].len());
        let (mut left, mut right) = (0, m * n);
        while left < right {
            let mid = left + (right - left) / 2;
            match matrix[mid / n][mid % n].cmp(&target) {
                Equal => return true,
                Less => left = mid + 1,
                Greater => right = mid,
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
        let target = 3;
        let result1 = Solution::search_matrix_a(matrix.clone(), target);
        let result2 = Solution::search_matrix_b(matrix.clone(), target);
        let result3 = Solution::search_matrix_c(matrix.clone(), target);
        assert_eq!(result1, true);
        assert_eq!(result2, true);
        assert_eq!(result3, true);
    }

    #[test]
    fn case_02() {
        let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
        let target = 13;
        let result1 = Solution::search_matrix_a(matrix.clone(), target);
        let result2 = Solution::search_matrix_b(matrix.clone(), target);
        let result3 = Solution::search_matrix_c(matrix.clone(), target);
        assert_eq!(result1, false);
        assert_eq!(result2, false);
        assert_eq!(result3, false);
    }
}
