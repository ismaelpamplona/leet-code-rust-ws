struct Solution;
impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn backtrack(cur: &mut Vec<i32>, nums: &[i32], ans: &mut Vec<Vec<i32>>, i: usize) {
            if i > nums.len() {
                return;
            }
            ans.push(cur.to_vec());
            for j in i..nums.len() {
                cur.push(nums[j]);
                backtrack(cur, nums, ans, j + 1);
                cur.pop();
            }
        }
        let mut ans: Vec<Vec<i32>> = vec![];
        backtrack(&mut vec![], &nums, &mut ans, 0);
        ans.sort_by_key(|v| v.len());
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let nums = vec![1, 2, 3];
        let mut expected = vec![
            vec![],
            vec![1],
            vec![2],
            vec![1, 2],
            vec![3],
            vec![1, 3],
            vec![2, 3],
            vec![1, 2, 3],
        ];
        expected.sort_by_key(|v| v.len());
        let result1 = Solution::subsets(nums.clone());
        assert_eq!(result1, expected);
    }

    #[test]
    fn case_02() {
        let nums = vec![0];
        let mut expected = vec![vec![], vec![0]];
        expected.sort_by_key(|v| v.len());
        let result1 = Solution::subsets(nums.clone());
        assert_eq!(result1, expected);
    }
}
