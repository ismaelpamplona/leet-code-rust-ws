struct Solution;
impl Solution {
    fn backtrack(
        start: usize,
        cur: &mut Vec<i32>,
        result: &mut Vec<Vec<i32>>,
        candidates: &Vec<i32>,
        sum: i32,
        target: i32,
    ) {
        if sum == target {
            result.push(cur.to_vec());
            return;
        }
        for i in start..candidates.len() {
            let n = candidates[i];
            if sum + n <= target {
                cur.push(n);
                Self::backtrack(i, cur, result, candidates, sum + n, target);
                cur.pop();
            }
        }
    }
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result = vec![];
        Self::backtrack(0, &mut vec![], &mut result, &candidates, 0, target);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let candidates = vec![2, 3, 6, 7];
        let result = Solution::combination_sum(candidates, 7);
        let expected = vec![vec![2, 2, 3], vec![7]];
        assert_eq!(result, expected);
    }

    #[test]
    fn case_02() {
        let candidates = vec![2, 3, 5];
        let result = Solution::combination_sum(candidates, 8);
        let expected = vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]];
        assert_eq!(result, expected);
    }

    #[test]
    fn case_03() {
        let candidates = vec![2];
        let result = Solution::combination_sum(candidates, 1);
        let expected: Vec<Vec<i32>> = vec![];
        assert_eq!(result, expected);
    }

    #[test]
    fn case_04() {
        let candidates = vec![7, 3, 2];
        let result = Solution::combination_sum(candidates, 18);
        let expected: Vec<Vec<i32>> = vec![
            vec![7, 7, 2, 2],
            vec![7, 3, 3, 3, 2],
            vec![7, 3, 2, 2, 2, 2],
            vec![3, 3, 3, 3, 3, 3],
            vec![3, 3, 3, 3, 2, 2, 2],
            vec![3, 3, 2, 2, 2, 2, 2, 2],
            vec![2, 2, 2, 2, 2, 2, 2, 2, 2],
        ];
        assert_eq!(result, expected);
    }
}
