use std::collections::{HashSet, VecDeque};

struct Solution;
impl Solution {
    pub fn nearest_exit(maze: Vec<Vec<char>>, entrance: Vec<i32>) -> i32 {
        fn is_exit(
            row: usize,
            col: usize,
            entrance: (usize, usize),
            maze: &Vec<Vec<char>>,
        ) -> bool {
            return (row == 0 || col == 0 || row == maze.len() - 1 || col == maze[0].len() - 1)
                && (row, col) != entrance;
        }
        fn is_valid(row: i32, col: i32, maze: &Vec<Vec<char>>) -> bool {
            return row >= 0
                && col >= 0
                && row < maze.len() as i32
                && col < maze[0].len() as i32
                && maze[row as usize][col as usize] == '.';
        }
        let mut maze = maze.clone();
        let dir: Vec<(i32, i32)> = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
        let entrance = (entrance[0] as usize, entrance[1] as usize);
        let mut q = VecDeque::from([(entrance.0, entrance.1, 0)]);
        while let Some((row, col, steps)) = q.pop_front() {
            if is_exit(row, col, entrance, &maze) {
                return steps;
            }
            for (dx, dy) in &dir {
                let (next_row, next_col) = (row as i32 + dy, col as i32 + dx);
                if is_valid(next_row, next_col, &maze) {
                    q.push_back((next_row as usize, next_col as usize, steps + 1));
                    maze[next_row as usize][next_col as usize] = '+';
                }
            }
        }
        -1
    }

    pub fn nearest_exit_seen(maze: Vec<Vec<char>>, entrance: Vec<i32>) -> i32 {
        fn is_exit(
            row: usize,
            col: usize,
            entrance: (usize, usize),
            maze: &Vec<Vec<char>>,
        ) -> bool {
            return (row == 0 || col == 0 || row == maze.len() - 1 || col == maze[0].len() - 1)
                && (row, col) != entrance;
        }
        fn is_valid(
            row: i32,
            col: i32,
            maze: &Vec<Vec<char>>,
            seen: &HashSet<(usize, usize)>,
        ) -> bool {
            return row >= 0
                && col >= 0
                && row < maze.len() as i32
                && col < maze[0].len() as i32
                && maze[row as usize][col as usize] == '.'
                && !seen.contains(&(row as usize, col as usize));
        }
        let dir: Vec<(i32, i32)> = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
        let entrance = (entrance[0] as usize, entrance[1] as usize);
        let mut seen = HashSet::from([entrance]);
        let mut q = VecDeque::from([(entrance.0, entrance.1, 0)]);
        while let Some((row, col, steps)) = q.pop_front() {
            if is_exit(row, col, entrance, &maze) {
                return steps;
            }
            for (dx, dy) in &dir {
                let (next_row, next_col) = (row as i32 + dy, col as i32 + dx);
                if is_valid(next_row, next_col, &maze, &seen) {
                    seen.insert((next_row as usize, next_col as usize));
                    q.push_back((next_row as usize, next_col as usize, steps + 1));
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
        let maze = vec![
            vec!['+', '+', '.', '+'],
            vec!['.', '.', '.', '+'],
            vec!['+', '+', '+', '.'],
        ];
        let entrance = vec![1, 2];
        let result1 = Solution::nearest_exit(maze.clone(), entrance.clone());
        let result2 = Solution::nearest_exit_seen(maze.clone(), entrance.clone());
        assert_eq!(result1, 1);
        assert_eq!(result2, 1);
    }

    #[test]
    fn case_02() {
        let maze = vec![
            vec!['+', '+', '+'],
            vec!['.', '.', '.'],
            vec!['+', '+', '+'],
        ];
        let entrance = vec![1, 0];
        let result1 = Solution::nearest_exit(maze.clone(), entrance.clone());
        let result2 = Solution::nearest_exit_seen(maze.clone(), entrance.clone());
        assert_eq!(result1, 2);
        assert_eq!(result2, 2);
    }

    #[test]
    fn case_03() {
        let maze = vec![vec!['.', '+']];
        let entrance = vec![0, 0];
        let result1 = Solution::nearest_exit(maze.clone(), entrance.clone());
        let result2 = Solution::nearest_exit_seen(maze.clone(), entrance.clone());
        assert_eq!(result1, -1);
        assert_eq!(result2, -1);
    }
}
