use std::collections::{HashMap, HashSet};

struct Solution;
impl Solution {
    pub fn count_components_it(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        fn dfs(node: i32, seen: &mut HashSet<i32>, graph: &HashMap<i32, Vec<i32>>) {
            let mut stack = vec![node];
            while let Some(node) = stack.pop() {
                if let Some(vec) = graph.get(&node) {
                    for neighbour in vec {
                        if !seen.contains(neighbour) {
                            seen.insert(*neighbour);
                            stack.push(*neighbour);
                        }
                    }
                }
            }
        }
        let mut graph = HashMap::new();
        for edge in edges {
            graph.entry(edge[0]).or_insert(vec![]).push(edge[1]);
            graph.entry(edge[1]).or_insert(vec![]).push(edge[0]);
        }
        let mut seen = HashSet::new();
        let mut number = 0;
        for i in 0..n {
            if !seen.contains(&i) {
                number += 1;
                seen.insert(i);
                dfs(i, &mut seen, &graph);
            }
        }
        number
    }

    pub fn count_components_rec(n: i32, edges: Vec<Vec<i32>>) -> i32 {
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
        for edge in edges {
            graph.entry(edge[0]).or_insert(vec![]).push(edge[1]);
            graph.entry(edge[1]).or_insert(vec![]).push(edge[0]);
        }
        let mut seen = HashSet::new();
        let mut number = 0;
        for i in 0..n {
            if !seen.contains(&i) {
                number += 1;
                seen.insert(i);
                dfs(i, &mut seen, &graph);
            }
        }
        number
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let edges = vec![vec![0, 1], vec![1, 2], vec![3, 4]];
        let result1 = Solution::count_components_it(5, edges.clone());
        let result2 = Solution::count_components_rec(5, edges.clone());
        assert_eq!(result1, 2);
        assert_eq!(result2, 2);
    }

    #[test]
    fn case_02() {
        let edges = vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4]];
        let result1 = Solution::count_components_it(5, edges.clone());
        let result2 = Solution::count_components_rec(5, edges.clone());
        assert_eq!(result1, 1);
        assert_eq!(result2, 1);
    }
}
