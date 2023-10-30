struct Solution;
impl Solution {
    fn backtrack(cur: &mut Vec<i32>, result: &mut Vec<Vec<i32>>, n: i32, k: i32, i: i32) {
        if cur.len() == k as usize {
            result.push(cur.to_vec());
            return;
        }
        for num in i..=n {
            cur.push(num);
            Self::backtrack(cur, result, n, k, num + 1);
            cur.pop();
        }
    }
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut result = vec![];
        Self::backtrack(&mut vec![], &mut result, n, k, 1);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let result = Solution::combine(4, 2);
        let mut expected = vec![
            vec![1, 2],
            vec![1, 3],
            vec![1, 4],
            vec![2, 3],
            vec![2, 4],
            vec![3, 4],
        ];
        expected.sort_by_key(|v| v.len());
        assert_eq!(result, expected);
    }

    #[test]
    fn case_02() {
        let result = Solution::combine(1, 1);
        let mut expected = vec![vec![1]];
        expected.sort_by_key(|v| v.len());
        assert_eq!(result, expected);
    }
}
