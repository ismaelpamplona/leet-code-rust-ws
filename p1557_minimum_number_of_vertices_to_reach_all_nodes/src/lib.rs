struct Solution;
impl Solution {
    pub fn find_smallest_set_of_vertices(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut indegree = vec![0; n as usize];
        let mut result = vec![];
        for e in edges {
            indegree[e[1] as usize] += 1;
        }
        for i in 0..n {
            if indegree[i as usize] == 0 {
                result.push(i as i32);
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let edges = vec![vec![0, 1], vec![0, 2], vec![2, 5], vec![3, 4], vec![4, 2]];
        let result1 = Solution::find_smallest_set_of_vertices(6, edges);
        assert_eq!(result1, vec![0, 3]);
    }

    #[test]
    fn case_02() {
        let edges = vec![vec![0, 1], vec![2, 1], vec![3, 1], vec![1, 4], vec![2, 4]];
        let result1 = Solution::find_smallest_set_of_vertices(5, edges);
        assert_eq!(result1, vec![0, 2, 3]);
    }

    #[test]
    fn case_03() {
        let edges = vec![vec![1, 2], vec![1, 0], vec![0, 2]];
        let result1 = Solution::find_smallest_set_of_vertices(3, edges);
        assert_eq!(result1, vec![1]);
    }
}
