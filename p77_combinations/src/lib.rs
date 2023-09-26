struct Solution;
impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        fn backtrack(cur: &mut Vec<i32>, n: i32, k: usize, ans: &mut Vec<Vec<i32>>, i: i32) {
            if k == cur.len() {
                ans.push(cur.to_vec());
                return;
            }
            for num in i..=n {
                cur.push(num);
                backtrack(cur, n, k, ans, num + 1);
                cur.pop();
            }
        }
        let mut ans: Vec<Vec<i32>> = vec![];
        backtrack(&mut vec![], n, k as usize, &mut ans, 1);
        ans.sort_by_key(|v| v.len());
        ans
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
