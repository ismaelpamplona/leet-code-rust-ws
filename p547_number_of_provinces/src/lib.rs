use std::collections::{HashMap, HashSet};

struct Solution;
impl Solution {
    pub fn find_circle_num_dfs_rec(is_connected: Vec<Vec<i32>>) -> i32 {
        fn dfs(node: usize, seen: &mut HashSet<usize>, graph: &HashMap<usize, Vec<usize>>) {
            if let Some(vec) = graph.get(&node) {
                for neighbor in vec {
                    if !seen.contains(neighbor) {
                        seen.insert(*neighbor);
                        dfs(*neighbor, seen, graph);
                    }
                }
            }
        }
        let n = is_connected.len();
        let mut graph = HashMap::new();
        for i in 0..n {
            for j in (i + 1)..n {
                if is_connected[i][j] == 1 {
                    graph.entry(i).or_insert(vec![]).push(j);
                    graph.entry(j).or_insert(vec![]).push(i);
                }
            }
        }
        let mut seen = HashSet::new();
        let mut connected = 0;
        for i in 0..n {
            if !seen.contains(&i) {
                connected += 1;
                seen.insert(i);
                dfs(i, &mut seen, &graph);
            }
        }
        connected
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let is_connected = vec![vec![1, 1, 0], vec![1, 1, 0], vec![0, 0, 1]];
        let result1 = Solution::find_circle_num_dfs_rec(is_connected);
        assert_eq!(result1, 2);
    }

    #[test]
    fn case_02() {
        let is_connected = vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]];
        let result1 = Solution::find_circle_num_dfs_rec(is_connected);
        assert_eq!(result1, 3);
    }

    #[test]
    fn case_03() {
        let is_connected = vec![
            vec![1, 0, 0, 1],
            vec![0, 1, 1, 0],
            vec![0, 1, 1, 1],
            vec![1, 0, 1, 1],
        ];
        let result1 = Solution::find_circle_num_dfs_rec(is_connected);
        assert_eq!(result1, 1);
    }
}
