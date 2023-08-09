use std::collections::{HashMap, HashSet, VecDeque};

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

    fn dfs_rec(node: usize, seen: &mut HashSet<usize>, graph: &HashMap<usize, Vec<usize>>) {
        if let Some(vec) = graph.get(&node) {
            println!("{:?}", node);
            for neighbor in vec {
                if !seen.contains(neighbor) {
                    seen.insert(*neighbor);
                    Self::dfs_rec(*neighbor, seen, graph);
                }
            }
        }
    }

    fn dfs_it(node: usize, seen: &mut HashSet<usize>, graph: &HashMap<usize, Vec<usize>>) {
        let mut stack = Vec::from([node]);
        while let Some(n) = stack.pop() {
            println!("{:?}", n);
            if let Some(vec) = graph.get(&n) {
                for neighbor in vec {
                    if !seen.contains(neighbor) {
                        seen.insert(*neighbor);
                        stack.push(*neighbor);
                    }
                }
            }
        }
    }

    fn bfs(grid: Vec<Vec<i32>>) -> i32 {
        if grid[0][0] == 1 {
            return -1;
        }
        let mut seen: HashSet<(usize, usize)> = HashSet::from([(0, 0)]);
        let mut q: VecDeque<(usize, usize, i32)> = VecDeque::from([(0, 0, 1)]);
        let dir: Vec<(i32, i32)> = vec![
            (0, 1), // to bottom
            (1, 0), // to right
            (1, 1), // to right and bottom
        ];
        while let Some((row, col, steps)) = q.pop_front() {
            let n = grid.len();
            if (row, col) == (n - 1, n - 1) {
                return steps;
            }
            for (dx, dy) in &dir {
                let (next_row, next_col) = ((row as i32 + dy) as usize, (col as i32 + dx) as usize);
                if grid[row][col] == 0 && !seen.contains(&(next_row, next_col)) {
                    seen.insert((next_row, next_col));
                    q.push_back((next_row, next_col, steps + 1));
                }
            }
        }
        -1
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

    #[test]
    fn case_dfs() {
        let matrix = vec![vec![1, 1, 0], vec![1, 1, 0], vec![0, 0, 1]];
        let n = matrix.len();
        let mut graph = HashMap::new();
        for i in 0..n {
            for j in (i + 1)..n {
                if matrix[i][j] == 1 {
                    graph.entry(i).or_insert(vec![]).push(j);
                    graph.entry(j).or_insert(vec![]).push(i);
                }
            }
        }
        let mut seen = HashSet::new();
        for i in 0..n {
            if !seen.contains(&i) {
                seen.insert(i);
                Solution::dfs_it(i, &mut seen, &graph);
                Solution::dfs_rec(i, &mut seen, &graph);
            }
        }
    }

    #[test]
    fn case_bfs() {
        let grid = vec![vec![0, 1], vec![1, 0]];
        let result1 = Solution::bfs(grid.clone());
        assert_eq!(result1, 2);
    }
}
