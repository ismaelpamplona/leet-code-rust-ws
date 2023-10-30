struct Solution;
impl Solution {
    fn backtrack(
        graph: &Vec<Vec<i32>>,
        result: &mut Vec<Vec<i32>>,
        path: &mut Vec<i32>,
        target: usize,
        cur: usize,
    ) {
        if cur == target {
            result.push(path.to_vec());
            return;
        }
        for next in &graph[cur] {
            path.push(*next);
            Self::backtrack(graph, result, path, target, *next as usize);
            path.pop();
        }
    }
    pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut result = vec![];
        let target = graph.len() - 1;
        let mut path = vec![0];
        Self::backtrack(&graph, &mut result, &mut path, target, 0);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let graph = vec![vec![1, 2], vec![3], vec![3], vec![]];
        let result = Solution::all_paths_source_target(graph);
        let mut expected: Vec<Vec<i32>> = vec![vec![0, 1, 3], vec![0, 2, 3]];
        assert_eq!(result, expected);
    }

    #[test]
    fn case_02() {
        let graph = vec![vec![4, 3, 1], vec![3, 2, 4], vec![3], vec![4], vec![]];
        let result = Solution::all_paths_source_target(graph);
        let mut expected: Vec<Vec<i32>> = vec![
            vec![0, 4],
            vec![0, 3, 4],
            vec![0, 1, 3, 4],
            vec![0, 1, 2, 3, 4],
            vec![0, 1, 4],
        ];
        assert_eq!(result, expected);
    }
}
