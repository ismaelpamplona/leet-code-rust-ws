use std::collections::{HashSet, VecDeque};

#[allow(dead_code)]
struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn minimum_effort_path(heights: Vec<Vec<i32>>) -> i32 {
        fn is_valid(x: i32, y: i32, m: i32, n: i32) -> bool {
            x >= 0 && x <= m && y >= 0 && y <= n
        }
        fn check(effort: i32, heights: &Vec<Vec<i32>>) -> bool {
            let (m, n) = (heights.len() as i32 - 1, heights[0].len() as i32 - 1);
            let mut q = VecDeque::from([(0, 0)]);
            let mut seen = HashSet::from([(0, 0)]);
            let directions: Vec<(i32, i32)> = vec![(0, -1), (0, 1), (-1, 0), (1, 0)];
            while let Some((row, col)) = q.pop_front() {
                if row == m && col == n {
                    return true;
                }
                for (dx, dy) in &directions {
                    let (nr, nc) = (row + dy, col + dx);
                    if is_valid(nr, nc, m, n)
                        && !seen.contains(&(nr, nc))
                        && (heights[nr as usize][nc as usize] - heights[row as usize][col as usize])
                            .abs()
                            <= effort
                    {
                        seen.insert((nr, nc));
                        q.push_back((nr, nc));
                    }
                }
            }
            false
        }
        let mut left = 0;
        let mut right = *heights
            .iter()
            .map(|row| row.iter().max())
            .filter_map(|x| x)
            .max()
            .unwrap();
        while left <= right {
            let mid = (left + right) / 2;
            if check(mid, &heights) {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }
        left
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let heights = vec![vec![1, 2, 2], vec![3, 8, 2], vec![5, 3, 5]];
        let result1 = Solution::minimum_effort_path(heights.clone());
        assert_eq!(result1, 2);
    }

    #[test]
    fn case_02() {
        let heights = vec![vec![1, 2, 3], vec![3, 8, 4], vec![5, 3, 5]];
        let result1 = Solution::minimum_effort_path(heights.clone());
        assert_eq!(result1, 1);
    }

    #[test]
    fn case_03() {
        let heights = vec![
            vec![1, 2, 1, 1, 1],
            vec![1, 2, 1, 2, 1],
            vec![1, 2, 1, 2, 1],
            vec![1, 2, 1, 2, 1],
            vec![1, 1, 1, 2, 1],
        ];
        let result1 = Solution::minimum_effort_path(heights.clone());
        assert_eq!(result1, 0);
    }
}
