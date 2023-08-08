use std::collections::HashSet;
struct Solution;
impl Solution {
    pub fn max_area_of_island_it(grid: Vec<Vec<i32>>) -> i32 {
        fn is_valid(node: (i32, i32), grid: &Vec<Vec<i32>>) -> bool {
            let (x, y) = node;
            let m = grid[0].len() as i32;
            let n = grid.len() as i32;
            return (0 <= x && 0 <= y && x < n && y < m)
                && grid[node.0 as usize][node.1 as usize] == 1;
        }
        fn dfs(
            node: (i32, i32),
            grid: &Vec<Vec<i32>>,
            seen: &mut HashSet<(i32, i32)>,
            directions: &Vec<(i32, i32)>,
            max_area: &mut i32,
        ) {
            let mut stack = vec![node];
            let mut cur_area = 0;
            while let Some(node) = stack.pop() {
                let (x, y) = node;
                seen.insert(node);
                cur_area += 1;
                for dir in directions {
                    let (nx, ny) = dir;
                    let next_node = (x + nx, y + ny);
                    if is_valid(next_node, grid) && !seen.contains(&next_node) {
                        stack.push(next_node);
                        seen.insert(next_node);
                    }
                }
            }
            *max_area = *max_area.max(&mut cur_area);
        }
        let directions = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];
        let mut seen = HashSet::new();
        let mut max_area = 0;
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if !seen.contains(&(i as i32, j as i32)) && grid[i][j] == 1 {
                    dfs(
                        (i as i32, j as i32),
                        &grid,
                        &mut seen,
                        &directions,
                        &mut max_area,
                    );
                }
            }
        }
        max_area
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let grid = vec![
            vec![0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
            vec![0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0],
            vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0],
        ];
        let result1 = Solution::max_area_of_island_it(grid.clone());
        assert_eq!(result1, 6);
    }

    #[test]
    fn case_02() {
        let grid = vec![vec![0, 0, 0, 0, 0, 0, 0, 0]];
        let result1 = Solution::max_area_of_island_it(grid.clone());
        assert_eq!(result1, 0);
    }

    #[test]
    fn case_03() {
        let grid = vec![vec![1]];
        let result1 = Solution::max_area_of_island_it(grid.clone());
        assert_eq!(result1, 1);
    }
}
