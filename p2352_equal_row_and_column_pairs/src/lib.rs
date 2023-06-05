use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
        let mut cols: Vec<Vec<i32>> = grid.clone();
        let mut map: HashMap<Vec<i32>, i32> = HashMap::new();
        let mut ans = 0;

        for i in 0..grid.len() {
            let entry = map.entry(grid[i].clone()).or_insert(0);
            *entry += 1;

            for j in 0..grid[i].len() {
                cols[j][i] = grid[i][j];
            }
        }

        for i in 0..cols.len() {
            let entry = map.entry(cols[i].clone()).or_insert(0);

            if *entry >= 1 {
                ans += *entry;
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let grid = vec![vec![3, 2, 1], vec![1, 7, 6], vec![2, 7, 7]];
        let result1 = Solution::equal_pairs(grid.clone());
        assert_eq!(result1, 1);
    }

    #[test]
    fn case_02() {
        let grid = vec![
            vec![3, 1, 2, 2],
            vec![1, 4, 4, 5],
            vec![2, 4, 2, 2],
            vec![2, 4, 2, 2],
        ];
        let result1 = Solution::equal_pairs(grid.clone());
        assert_eq!(result1, 3);
    }

    #[test]
    fn case_03() {
        let grid = vec![
            vec![3, 1, 2, 2],
            vec![1, 4, 4, 4],
            vec![2, 4, 2, 2],
            vec![2, 5, 2, 2],
        ];
        let result1 = Solution::equal_pairs(grid.clone());
        assert_eq!(result1, 3);
    }

    #[test]
    fn case_04() {
        let grid = vec![vec![11, 1], vec![1, 11]];
        let result1 = Solution::equal_pairs(grid.clone());
        assert_eq!(result1, 2);
    }
}
