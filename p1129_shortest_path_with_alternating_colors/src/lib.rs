use std::collections::{HashMap, HashSet, VecDeque};

struct Solution;
impl Solution {
    pub fn shortest_alternating_paths(
        n: i32,
        red_edges: Vec<Vec<i32>>,
        blue_edges: Vec<Vec<i32>>,
    ) -> Vec<i32> {
        let red = 0;
        let blue = 1;
        let mut graph: HashMap<i32, HashMap<i32, Vec<i32>>> = HashMap::new();
        fn add_to_graph(
            graph: &mut HashMap<i32, HashMap<i32, Vec<i32>>>,
            edges: Vec<Vec<i32>>,
            color: i32,
        ) {
            for edge in edges {
                graph
                    .entry(color)
                    .or_insert(HashMap::new())
                    .entry(edge[0])
                    .or_insert(vec![])
                    .push(edge[1]);
            }
        }
        add_to_graph(&mut graph, red_edges, red);
        add_to_graph(&mut graph, blue_edges, blue);
        let mut result = vec![i32::MAX; n as usize];
        let mut q = VecDeque::from([(0, red, 0), (0, blue, 0)]);
        let mut seen = HashSet::from([(0, red), (0, blue)]);
        while let Some((node, color, steps)) = q.pop_front() {
            result[node as usize] = result[node as usize].min(steps);
            if let Some(map) = graph.get(&color) {
                if let Some(vec) = map.get(&(node)) {
                    for neighbour in vec {
                        if !seen.contains(&(*neighbour, 1 - color)) {
                            seen.insert((*neighbour, 1 - color));
                            q.push_back((*neighbour, 1 - color, steps + 1));
                        }
                    }
                }
            }
        }
        result.iter_mut().for_each(|x| {
            *x = if *x == i32::MAX { -1 } else { *x };
        });
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let red_edges = vec![vec![0, 1], vec![1, 2]];
        let blue_edges = vec![];
        let expected = vec![0, 1, -1];
        let result1 = Solution::shortest_alternating_paths(3, red_edges, blue_edges);
        assert_eq!(result1, expected);
    }

    #[test]
    fn case_02() {
        let red_edges = vec![vec![0, 1]];
        let blue_edges = vec![vec![2, 1]];
        let expected = vec![0, 1, -1];
        let result1 = Solution::shortest_alternating_paths(3, red_edges, blue_edges);
        assert_eq!(result1, expected);
    }

    #[test]
    fn case_03() {
        let red_edges = vec![vec![0, 1]];
        let blue_edges = vec![vec![1, 2]];
        let expected = vec![0, 1, 2];
        let result1 = Solution::shortest_alternating_paths(3, red_edges, blue_edges);
        assert_eq!(result1, expected);
    }
}
