use std::collections::{HashMap, HashSet};
struct Solution;
impl Solution {
    pub fn min_reorder_rec(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        fn dfs(
            node: i32,
            graph: &HashMap<i32, Vec<i32>>,
            seen: &mut HashSet<i32>,
            roads: &HashSet<Vec<i32>>,
        ) -> i32 {
            let mut ans = 0;
            if let Some(vec) = graph.get(&node) {
                for neighbour in vec {
                    if !seen.contains(neighbour) {
                        if roads.contains(&vec![node, *neighbour]) {
                            ans += 1;
                        }
                        seen.insert(*neighbour);
                        ans += dfs(*neighbour, graph, seen, roads);
                    }
                }
            }
            ans
        }
        let mut roads = HashSet::new();
        let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
        for road in connections {
            graph.entry(road[0]).or_insert(vec![]).push(road[1]);
            graph.entry(road[1]).or_insert(vec![]).push(road[0]);
            roads.insert(road);
        }
        let mut seen = HashSet::from([0]);
        return dfs(0, &graph, &mut seen, &roads);
    }

    pub fn min_reorder_it(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        let mut roads = HashSet::new();
        let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
        for road in connections {
            graph.entry(road[0]).or_insert(vec![]).push(road[1]);
            graph.entry(road[1]).or_insert(vec![]).push(road[0]);
            roads.insert(road);
        }
        let mut seen = HashSet::from([0]);
        let mut stack = vec![0];
        let mut ans = 0;
        while let Some(node) = stack.pop() {
            if let Some(vec) = graph.get(&node) {
                for neighbour in vec {
                    if !seen.contains(neighbour) {
                        if roads.contains(&vec![node, *neighbour]) {
                            ans += 1;
                        }
                        seen.insert(*neighbour);
                        stack.push(*neighbour);
                    }
                }
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
        let connections = vec![vec![0, 1], vec![1, 3], vec![2, 3], vec![4, 0], vec![4, 5]];
        let result1 = Solution::min_reorder_rec(6, connections.clone());
        let result2 = Solution::min_reorder_it(6, connections.clone());
        assert_eq!(result1, 3);
        assert_eq!(result2, 3);
    }

    #[test]
    fn case_02() {
        let connections = vec![vec![1, 0], vec![1, 2], vec![3, 2], vec![3, 4]];
        let result1 = Solution::min_reorder_rec(5, connections.clone());
        let result2 = Solution::min_reorder_it(5, connections.clone());
        assert_eq!(result1, 2);
        assert_eq!(result2, 2);
    }

    #[test]
    fn case_03() {
        let connections = vec![vec![1, 0], vec![2, 0]];
        let result1 = Solution::min_reorder_rec(3, connections.clone());
        let result2 = Solution::min_reorder_it(3, connections.clone());
        assert_eq!(result1, 0);
        assert_eq!(result2, 0);
    }
}
