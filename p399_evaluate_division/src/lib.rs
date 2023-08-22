use std::collections::{HashMap, HashSet};

struct Solution;
impl Solution {
    pub fn calc_equation(
        equations: Vec<Vec<String>>,
        values: Vec<f64>,
        queries: Vec<Vec<String>>,
    ) -> Vec<f64> {
        fn answer_query(
            start: &String,
            end: &String,
            graph: &HashMap<&String, HashMap<&String, f64>>,
        ) -> f64 {
            if !graph.contains_key(start) {
                return -1.0;
            }
            let mut seen = HashSet::new();
            seen.insert(start);
            let mut stack = vec![(start, 1.0)];
            while let Some((node, ratio)) = stack.pop() {
                if node == end {
                    return ratio;
                }
                if let Some(neighbours) = graph.get(node) {
                    // code
                    for neighbour in neighbours {}
                }
            }
            -1.0
        }
        let mut graph = HashMap::new();
        for i in 0..equations.len() {
            let n = &equations[i][0];
            let d = &equations[i][1];
            let v = values[i];
            graph.entry(n).or_insert(HashMap::new()).insert(d, v);
            graph.entry(d).or_insert(HashMap::new()).insert(n, 1.0 / v);
        }
        let mut result = vec![];
        for q in queries {
            // result.push()
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let equations = vec![
            vec![String::from("a"), String::from("b")],
            vec![String::from("b"), String::from("c")],
        ];
        let values = vec![2.0, 3.0];
        let queries = vec![
            vec![String::from("a"), String::from("c")],
            vec![String::from("b"), String::from("a")],
            vec![String::from("a"), String::from("e")],
            vec![String::from("a"), String::from("a")],
            vec![String::from("x"), String::from("x")],
        ];
        let expected = vec![6.00000, 0.50000, -1.00000, 1.00000, -1.00000];
        let result1 = Solution::calc_equation(equations, values, queries);
        assert_eq!(result1, expected);
    }

    #[test]
    fn case_02() {
        let equations = vec![
            vec![String::from("a"), String::from("b")],
            vec![String::from("b"), String::from("c")],
            vec![String::from("bc"), String::from("cd")],
        ];
        let values = vec![1.5, 2.5, 5.0];
        let queries = vec![
            vec![String::from("a"), String::from("c")],
            vec![String::from("c"), String::from("b")],
            vec![String::from("bc"), String::from("cd")],
            vec![String::from("cd"), String::from("bc")],
        ];
        let expected = vec![3.75000, 0.40000, 5.00000, 0.20000];
        let result1 = Solution::calc_equation(equations, values, queries);
        assert_eq!(result1, expected);
    }

    #[test]
    fn case_03() {
        let equations = vec![vec![String::from("a"), String::from("b")]];
        let values = vec![0.5];
        let queries = vec![
            vec![String::from("a"), String::from("b")],
            vec![String::from("b"), String::from("a")],
            vec![String::from("a"), String::from("c")],
            vec![String::from("x"), String::from("y")],
        ];
        let expected = vec![0.50000, 2.00000, -1.00000, -1.00000];
        let result1 = Solution::calc_equation(equations, values, queries);
        assert_eq!(result1, expected);
    }
}
