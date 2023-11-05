use std::collections::HashSet;

struct Solution;
impl Solution {
    fn backtrack(
        n: i32,
        row: i32,
        diagonals: &mut HashSet<i32>,
        anti_diagonals: &mut HashSet<i32>,
        cols: &mut HashSet<i32>,
    ) -> i32 {
        if row == n {
            return 1;
        }
        let mut solutions = 0;
        for col in 0..n {
            let cur_diagonal = row - col;
            let cur_anti_diagonal = row + col;
            if cols.contains(&col)
                || diagonals.contains(&cur_diagonal)
                || anti_diagonals.contains(&cur_anti_diagonal)
            {
                continue;
            }
            cols.insert(col);
            diagonals.insert(cur_diagonal);
            anti_diagonals.insert(cur_anti_diagonal);

            solutions += Self::backtrack(n, row + 1, diagonals, anti_diagonals, cols);

            cols.remove(&col);
            diagonals.remove(&cur_diagonal);
            anti_diagonals.remove(&cur_anti_diagonal);
        }
        solutions
    }
    pub fn total_n_queens(n: i32) -> i32 {
        Self::backtrack(
            n,
            0,
            &mut HashSet::new(),
            &mut HashSet::new(),
            &mut HashSet::new(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let result = Solution::total_n_queens(4);
        assert_eq!(result, 2);
    }

    #[test]
    fn case_02() {
        let result = Solution::total_n_queens(1);
        assert_eq!(result, 1);
    }
}
