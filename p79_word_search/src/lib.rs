use std::collections::HashSet;

struct Solution;
impl Solution {
    fn is_valid(x: i32, y: i32, m: i32, n: i32) -> bool {
        0 <= x && 0 <= y && x < m && y < n
    }
    fn backtrack(
        cur_cel: (i32, i32),
        cur_char: usize,
        word: &mut Vec<char>,
        board: &Vec<Vec<char>>,
        dirs: &Vec<(i32, i32)>,
        set: &mut HashSet<(i32, i32)>,
        m: i32,
        n: i32,
    ) -> bool {
        if cur_char == word.len() {
            return true;
        }
        for (x, y) in dirs {
            let next = (cur_cel.0 + x, cur_cel.1 + y);
            if Self::is_valid(next.0, next.1, m, n) && !set.contains(&next) {
                if board[next.0 as usize][next.1 as usize] == word[cur_char] {
                    set.insert(next);
                    if Self::backtrack(next, cur_char + 1, word, board, dirs, set, m, n) {
                        return true;
                    }
                    set.remove(&next);
                }
            }
        }
        false
    }
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let mut word: Vec<char> = word.chars().collect();
        let dirs = vec![(0, 1), (0, -1), (-1, 0), (1, 0)];
        let m = board.len();
        let n = board[0].len();
        for row in 0..m {
            for col in 0..n {
                if board[row][col] == word[0]
                    && Self::backtrack(
                        (row as i32, col as i32),
                        1,
                        &mut word,
                        &board,
                        &dirs,
                        &mut HashSet::from([(row as i32, col as i32)]),
                        m as i32,
                        n as i32,
                    )
                {
                    return true;
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
    fn case_01() {
        let board = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];
        let word = "ABCCED".to_string();
        let result = Solution::exist(board, word);
        assert_eq!(result, true);
    }

    #[test]
    fn case_02() {
        let board = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];
        let word = "SEE".to_string();
        let result = Solution::exist(board, word);
        assert_eq!(result, true);
    }

    #[test]
    fn case_03() {
        let board = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];
        let word = "ABCB".to_string();
        let result = Solution::exist(board, word);
        assert_eq!(result, false);
    }
}
