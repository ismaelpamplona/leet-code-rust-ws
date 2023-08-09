use std::collections::{HashSet, VecDeque};

struct Solution;
impl Solution {
    pub fn shortest_path_binary_matrix(grid: Vec<Vec<i32>>) -> i32 {
        if grid[0][0] == 1 {
            return -1;
        }
        fn is_valid(row: usize, col: usize, grid: &Vec<Vec<i32>>) -> bool {
            let n = grid.len();
            return 0 <= row && row < n && 0 <= col && col < n && grid[row][col] == 0;
        }
        let mut seen: HashSet<(usize, usize)> = HashSet::from([(0, 0)]);
        let mut q: VecDeque<(usize, usize, i32)> = VecDeque::from([(0, 0, 1)]);
        let dir: Vec<(i32, i32)> = vec![
            (0, 1),
            (1, 0),
            (1, 1),
            (-1, -1),
            (-1, 1),
            (1, -1),
            (0, -1),
            (-1, 0),
        ];
        while let Some((row, col, steps)) = q.pop_front() {
            let n = grid.len();
            if (row, col) == (n - 1, n - 1) {
                return steps;
            }
            for (dx, dy) in &dir {
                let (next_row, next_col) = ((row as i32 + dy) as usize, (col as i32 + dx) as usize);
                if is_valid(next_row, next_col, &grid) && !seen.contains(&(next_row, next_col)) {
                    seen.insert((next_row, next_col));
                    q.push_back((next_row, next_col, steps + 1));
                }
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let grid = vec![vec![0, 1], vec![1, 0]];
        let result1 = Solution::shortest_path_binary_matrix(grid.clone());
        assert_eq!(result1, 2);
    }

    #[test]
    fn case_02() {
        let grid = vec![vec![0, 0, 0], vec![1, 1, 0], vec![1, 1, 0]];
        let result1 = Solution::shortest_path_binary_matrix(grid.clone());
        assert_eq!(result1, 4);
    }

    #[test]
    fn case_03() {
        let grid = vec![vec![1, 0, 0], vec![1, 1, 0], vec![1, 1, 0]];
        let result1 = Solution::shortest_path_binary_matrix(grid.clone());
        assert_eq!(result1, -1);
    }
}
