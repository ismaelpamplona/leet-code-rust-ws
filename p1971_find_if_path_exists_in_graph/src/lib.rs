use std::collections::{HashMap, HashSet};

struct Solution;
impl Solution {
    pub fn valid_path_it(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
        let mut graph = HashMap::new();
        for edge in &edges {
            graph.entry(edge[0]).or_insert(vec![]).push(edge[1]);
            graph.entry(edge[1]).or_insert(vec![]).push(edge[0]);
        }
        let mut seen = HashSet::from([source]);
        let mut stack = vec![source];
        while let Some(node) = stack.pop() {
            if let Some(vec) = graph.get(&node) {
                for neighbour in vec {
                    if !seen.contains(neighbour) {
                        seen.insert(*neighbour);
                        stack.push(*neighbour)
                    }
                }
            }
        }
        seen.contains(&destination)
    }

    pub fn valid_path_rec(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
        fn dfs(node: i32, seen: &mut HashSet<i32>, graph: &HashMap<i32, Vec<i32>>) {
            if let Some(vec) = graph.get(&node) {
                for neighbour in vec {
                    if !seen.contains(neighbour) {
                        seen.insert(*neighbour);
                        dfs(*neighbour, seen, graph);
                    }
                }
            }
        }
        let mut graph = HashMap::new();
        for edge in &edges {
            graph.entry(edge[0]).or_insert(vec![]).push(edge[1]);
            graph.entry(edge[1]).or_insert(vec![]).push(edge[0]);
        }
        let mut seen = HashSet::from([source]);
        dfs(source, &mut seen, &graph);
        seen.contains(&destination)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let edges = vec![vec![0, 1], vec![1, 2], vec![2, 0]];
        let result1 = Solution::valid_path_it(3, edges.clone(), 0, 2);
        let result2 = Solution::valid_path_rec(3, edges.clone(), 0, 2);
        assert_eq!(result1, true);
        assert_eq!(result2, true);
    }

    #[test]
    fn case_02() {
        let edges = vec![vec![0, 1], vec![0, 2], vec![3, 5], vec![5, 4], vec![4, 3]];
        let result1 = Solution::valid_path_it(6, edges.clone(), 0, 5);
        let result2 = Solution::valid_path_rec(6, edges.clone(), 0, 5);
        assert_eq!(result1, false);
        assert_eq!(result2, false);
    }

    #[test]
    fn case_03() {
        let edges = vec![vec![0, 4]];
        let result1 = Solution::valid_path_it(6, edges.clone(), 0, 4);
        let result2 = Solution::valid_path_rec(6, edges.clone(), 0, 4);
        assert_eq!(result1, true);
        assert_eq!(result2, true);
    }

    #[test]
    fn case_04() {
        let edges = vec![];
        let result1 = Solution::valid_path_it(1, edges.clone(), 0, 0);
        let result2 = Solution::valid_path_rec(1, edges.clone(), 0, 0);
        assert_eq!(result1, true);
        assert_eq!(result2, true);
    }
}
