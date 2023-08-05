use std::collections::HashSet;
struct Solution;
impl Solution {
    pub fn num_islands_rec(grid: Vec<Vec<char>>) -> i32 {
        fn is_valid(row: i32, col: i32, grid: &Vec<Vec<char>>) -> bool {
            0 <= row
                && 0 <= col
                && row < grid.len() as i32
                && col < grid[0].len() as i32
                && grid[row as usize][col as usize] == '1'
        }
        fn dfs(
            row: i32,
            col: i32,
            seen: &mut HashSet<(i32, i32)>,
            grid: &Vec<Vec<char>>,
            directions: &Vec<(i32, i32)>,
        ) {
            for (dx, dy) in directions {
                let (next_row, next_col) = (row + dy, col + dx);
                if is_valid(next_row, next_col, grid) && !seen.contains(&(next_row, next_col)) {
                    seen.insert((next_row, next_col));
                    dfs(next_row, next_col, seen, grid, directions);
                }
            }
        }
        let directions: Vec<(i32, i32)> = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
        let mut num_islands = 0;
        let mut seen = HashSet::new();
        for row in 0..grid.len() as i32 {
            for col in 0..grid[0].len() as i32 {
                if grid[row as usize][col as usize] == '1' && !seen.contains(&(row, col)) {
                    num_islands += 1;
                    seen.insert((row, col));
                    dfs(row, col, &mut seen, &grid, &directions);
                }
            }
        }
        num_islands
    }

    pub fn num_islands_it(grid: Vec<Vec<char>>) -> i32 {
        fn is_valid(row: i32, col: i32, grid: &Vec<Vec<char>>) -> bool {
            0 <= row
                && 0 <= col
                && row < grid.len() as i32
                && col < grid[0].len() as i32
                && grid[row as usize][col as usize] == '1'
        }
        fn dfs(
            row: i32,
            col: i32,
            seen: &mut HashSet<(i32, i32)>,
            grid: &Vec<Vec<char>>,
            directions: &Vec<(i32, i32)>,
        ) {
            let mut stack = vec![(row, col)];
            while let Some((row, col)) = stack.pop() {
                for (dx, dy) in directions {
                    let (next_row, next_col) = (row + dy, col + dx);
                    if is_valid(next_row, next_col, grid) && !seen.contains(&(next_row, next_col)) {
                        seen.insert((next_row, next_col));
                        stack.push((next_row, next_col));
                    }
                }
            }
        }
        let directions: Vec<(i32, i32)> = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
        let mut num_islands = 0;
        let mut seen = HashSet::new();
        for row in 0..grid.len() as i32 {
            for col in 0..grid[0].len() as i32 {
                if grid[row as usize][col as usize] == '1' && !seen.contains(&(row, col)) {
                    num_islands += 1;
                    seen.insert((row, col));
                    dfs(row, col, &mut seen, &grid, &directions);
                }
            }
        }
        num_islands
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let grid = vec![
            vec!['1', '1', '1', '1', '0'],
            vec!['1', '1', '0', '1', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '0', '0', '0'],
        ];
        let result1 = Solution::num_islands_rec(grid.clone());
        let result2 = Solution::num_islands_it(grid.clone());
        assert_eq!(result1, 1);
        assert_eq!(result2, 1);
    }

    #[test]
    fn case_02() {
        let grid = vec![
            vec!['1', '1', '0', '0', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '1', '0', '0'],
            vec!['0', '0', '0', '1', '1'],
        ];
        let result1 = Solution::num_islands_rec(grid.clone());
        let result2 = Solution::num_islands_it(grid.clone());
        assert_eq!(result1, 3);
        assert_eq!(result2, 3);
    }

    #[test]
    fn case_03() {
        let grid = vec![
            vec!['1', '1', '0', '0', '0', '1'],
            vec!['0', '1', '0', '0', '0', '0'],
            vec!['0', '1', '1', '0', '1', '1'],
            vec!['0', '0', '0', '0', '0', '1'],
            vec!['1', '1', '1', '1', '0', '1'],
            vec!['1', '1', '1', '1', '0', '1'],
        ];
        let result1 = Solution::num_islands_rec(grid.clone());
        let result2 = Solution::num_islands_it(grid.clone());
        assert_eq!(result1, 4);
        assert_eq!(result2, 4);
    }
}
