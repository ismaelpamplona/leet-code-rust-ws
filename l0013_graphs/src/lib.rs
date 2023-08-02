use std::collections::HashMap;

struct Solution;
impl Solution {
    fn build_graph_from_array_of_edges(edges: Vec<Vec<i32>>) -> HashMap<i32, Vec<i32>> {
        let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
        for edge in edges {
            let entry = graph.entry(edge[0]).or_insert(vec![]);
            entry.push(edge[1]);
        }
        graph
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_array_of_edges() {
        let edges = vec![vec![0, 1], vec![1, 2], vec![2, 0], vec![2, 3]];
        let graph1 = Solution::build_graph_from_array_of_edges(edges);
        println!("{:#?}", graph1);
    }
}
