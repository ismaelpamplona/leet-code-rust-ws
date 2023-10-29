struct Solution;
impl Solution {
    fn backtrack(cur: &mut Vec<i32>, nums: &Vec<i32>, result: &mut Vec<Vec<i32>>, i: usize) {
        if cur.len() > nums.len() {
            return;
        }
        result.push(cur.to_vec());

        for j in i..nums.len() {
            cur.push(nums[j]);
            Self::backtrack(cur, nums, result, j + 1);
            cur.pop();
        }
    }
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = vec![];
        Self::backtrack(&mut vec![], &nums, &mut result, 0);
        result.sort_by_key(|v| v.len());
        result
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
