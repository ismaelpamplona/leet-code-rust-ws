struct Solution;
impl Solution {
    fn backtrack(cur: &mut Vec<i32>, nums: &[i32], ans: &mut Vec<Vec<i32>>) {
        if cur.len() == nums.len() {
            ans.push(cur.to_vec());
            return;
        }
        for n in nums {
            if !cur.contains(n) {
                cur.push(*n);
                Self::backtrack(cur, nums, ans);
                cur.pop();
            }
        }
    }
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans: Vec<Vec<i32>> = vec![];
        Self::backtrack(&mut vec![], &nums, &mut ans);
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let nums = vec![1, 2, 3];
        let result = Solution::permute(nums);
        let expected = vec![
            vec![1, 2, 3],
            vec![1, 3, 2],
            vec![2, 1, 3],
            vec![2, 3, 1],
            vec![3, 1, 2],
            vec![3, 2, 1],
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn case_02() {
        let nums = vec![0, 1];
        let result = Solution::permute(nums);
        let expected = vec![vec![0, 1], vec![1, 0]];
        assert_eq!(result, expected);
    }

    #[test]
    fn case_03() {
        let nums = vec![1];
        let result = Solution::permute(nums);
        let expected = vec![vec![1]];
        assert_eq!(result, expected);
    }
}
