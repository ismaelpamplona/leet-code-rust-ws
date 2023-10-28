struct Solution;
impl Solution {
    fn backtrack(start: i32, cur: &mut Vec<i32>, n: i32, k: i32, results: &mut Vec<Vec<i32>>) {
        // Base case: if the combination is done
        if cur.len() == k as usize {
            results.push(cur.to_vec());
            return;
        }

        // Iterate over the numbers from start to n
        for i in start..=n {
            // Modify cur
            cur.push(i);

            // Recursively generate combinations for the next position
            Self::backtrack(i + 1, cur, n, k, results);

            // Undo the modification
            cur.pop();
        }
    }

    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut results = Vec::new();
        Self::backtrack(1, &mut Vec::new(), n, k, &mut results);
        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let n = 4;
        let k = 2;
        let result = Solution::combine(n, k);
        let expected = vec![
            vec![1, 2],
            vec![1, 3],
            vec![1, 4],
            vec![2, 3],
            vec![2, 4],
            vec![3, 4],
        ];
        assert_eq!(result, expected);
    }
}
