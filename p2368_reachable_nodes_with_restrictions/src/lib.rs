use std::collections::{HashMap, HashSet};

struct Solution;
impl Solution {
    pub fn reachable_nodes_it(n: i32, edges: Vec<Vec<i32>>, restricted: Vec<i32>) -> i32 {
        let mut restricted_set = HashSet::new();
        for r in restricted {
            restricted_set.insert(r);
        }
        let mut graph = HashMap::new();
        for i in 0..edges.len() {
            if !restricted_set.contains(&edges[i][0]) && !restricted_set.contains(&edges[i][1]) {
                graph.entry(edges[i][1]).or_insert(vec![]).push(edges[i][0]);
                graph.entry(edges[i][0]).or_insert(vec![]).push(edges[i][1]);
            }
        }
        let mut seen = HashSet::from([0]);
        let mut stack = vec![0];
        while let Some(node) = stack.pop() {
            if let Some(vec) = graph.get(&node) {
                for neighbour in vec {
                    if !seen.contains(neighbour) {
                        stack.push(*neighbour);
                        seen.insert(*neighbour);
                    }
                }
            }
        }
        seen.len() as i32
    }

    pub fn reachable_nodes_rec(n: i32, edges: Vec<Vec<i32>>, restricted: Vec<i32>) -> i32 {
        let mut restricted_set = HashSet::new();
        for r in restricted {
            restricted_set.insert(r);
        }
        let mut graph = HashMap::new();
        for i in 0..edges.len() {
            if !restricted_set.contains(&edges[i][0]) && !restricted_set.contains(&edges[i][1]) {
                graph.entry(edges[i][1]).or_insert(vec![]).push(edges[i][0]);
                graph.entry(edges[i][0]).or_insert(vec![]).push(edges[i][1]);
            }
        }
        let mut seen = HashSet::from([0]);
        fn dfs(node: i32, graph: &HashMap<i32, Vec<i32>>, seen: &mut HashSet<i32>) {
            if let Some(vec) = graph.get(&node) {
                for neighbour in vec {
                    if !seen.contains(neighbour) {
                        seen.insert(*neighbour);
                        dfs(*neighbour, graph, seen);
                    }
                }
            }
        }
        dfs(0, &graph, &mut seen);
        seen.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let n = 7;
        let edges = vec![
            vec![0, 1],
            vec![1, 2],
            vec![3, 1],
            vec![4, 0],
            vec![0, 5],
            vec![5, 6],
        ];
        let restricted = vec![4, 5];
        let result1 = Solution::reachable_nodes_it(n, edges.clone(), restricted.clone());
        let result2 = Solution::reachable_nodes_rec(n, edges.clone(), restricted.clone());
        assert_eq!(result1, 4);
        assert_eq!(result2, 4);
    }

    #[test]
    fn case_02() {
        let n = 7;
        let edges = vec![
            vec![0, 1],
            vec![0, 2],
            vec![0, 5],
            vec![0, 4],
            vec![3, 2],
            vec![6, 5],
        ];
        let restricted = vec![4, 2, 1];
        let result1 = Solution::reachable_nodes_it(n, edges.clone(), restricted.clone());
        let result2 = Solution::reachable_nodes_rec(n, edges.clone(), restricted.clone());
        assert_eq!(result1, 3);
        assert_eq!(result2, 3);
    }
}
