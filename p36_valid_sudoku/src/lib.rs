use std::collections::{HashMap, HashSet};

struct Solution;
impl Solution {
    fn has_in(map: &mut HashMap<usize, HashSet<char>>, i: usize, ch: char) -> bool {
        !map.entry(i).or_insert_with(HashSet::new).insert(ch)
    }
    fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut rows = HashMap::new();
        let mut cols = HashMap::new();
        let mut boxes = HashMap::new();

        for ri in 0..9 {
            for ci in 0..9 {
                let ch = board[ri][ci];
                if ch == '.' {
                    continue;
                }
                let box_index = 3 * (ri / 3) + (ci / 3);

                if Self::has_in(&mut rows, ri, ch)
                    || Self::has_in(&mut cols, ci, ch)
                    || Self::has_in(&mut boxes, box_index, ch)
                {
                    return false;
                }
            }
        }

        true
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let board = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        let result = Solution::is_valid_sudoku(board.clone());
        assert_eq!(result, true);
    }

    #[test]
    fn case_02() {
        let board = vec![
            vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        let result = Solution::is_valid_sudoku(board.clone());
        assert_eq!(result, false);
    }
}
