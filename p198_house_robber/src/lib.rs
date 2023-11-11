use std::cmp::max;
struct Solution;
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let n = nums.len() - 1;

        let mut rob_next_plus_one = 0;
        let mut rob_next = nums[n];

        for i in (0..n).rev() {
            let current = max(rob_next, rob_next_plus_one + nums[i]);
            rob_next_plus_one = rob_next;
            rob_next = current;
        }

        rob_next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let nums = vec![1, 2, 3, 1];
        let result = Solution::rob(nums);
        assert_eq!(result, 4);
    }

    #[test]
    fn case_02() {
        let nums = vec![2, 7, 9, 3, 1];
        let result = Solution::rob(nums);
        assert_eq!(result, 12);
    }
}
