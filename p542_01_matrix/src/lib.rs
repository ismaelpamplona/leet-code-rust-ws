use std::collections::{HashSet, VecDeque};

struct Solution;
impl Solution {
    pub fn update_matrix(mut mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        fn is_valid(row: i32, col: i32, mat: &Vec<Vec<i32>>) -> bool {
            return 0 <= row && 0 <= col && row < mat.len() as i32 && col < mat[0].len() as i32;
        }
        let mut q = VecDeque::with_capacity(mat.len() * mat[0].len());
        let mut seen = HashSet::new();
        for row in 0..mat.len() {
            for col in 0..mat[row].len() {
                if mat[row][col] == 0 {
                    q.push_back((row as i32, col as i32, 1));
                    seen.insert((row as i32, col as i32));
                }
            }
        }
        let dir: Vec<(i32, i32)> = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
        while let Some((row, col, steps)) = q.pop_front() {
            for (dx, dy) in &dir {
                let (next_row, next_col) = (row as i32 + *dy, col as i32 + *dx);
                let neighbour = (next_row, next_col);
                if is_valid(next_row, next_col, &mat) && !seen.contains(&neighbour) {
                    seen.insert(neighbour);
                    q.push_back((next_row, next_col, steps + 1));
                    mat[next_row as usize][next_col as usize] = steps;
                }
            }
        }
        mat
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let mat = vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]];
        let expected = vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]];
        let result1 = Solution::update_matrix(mat.clone());
        assert_eq!(result1, expected);
    }

    #[test]
    fn case_02() {
        let mat = vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 1, 1]];
        let expected = vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 2, 1]];
        let result1 = Solution::update_matrix(mat.clone());
        assert_eq!(result1, expected);
    }

    #[test]
    fn case_03() {
        let mat = vec![vec![0], vec![0], vec![0], vec![0], vec![0]];
        let expected = vec![vec![0], vec![0], vec![0], vec![0], vec![0]];
        let result1 = Solution::update_matrix(mat.clone());
        assert_eq!(result1, expected);
    }
}
