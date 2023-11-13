use std::{cmp::max, collections::HashMap};
struct Solution;
impl Solution {
    pub fn rob_bup(nums: Vec<i32>) -> i32 {
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
    fn dp(i: usize, nums: &[i32], cache: &mut HashMap<usize, i32>) -> i32 {
        if let Some(&cached) = cache.get(&i) {
            return cached;
        }

        let result = if i == 0 {
            nums[0]
        } else if i == 1 {
            nums[0].max(nums[1])
        } else {
            Self::dp(i - 1, nums, cache).max(Self::dp(i - 2, nums, cache) + nums[i])
        };

        cache.insert(i, result);
        result
    }
    pub fn rob_tdown(nums: Vec<i32>) -> i32 {
        let mut cache = HashMap::new();
        Self::dp(nums.len() - 1, &nums, &mut cache)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let nums = vec![1, 2, 3, 1];
        let result1 = Solution::rob_bup(nums.clone());
        let result2 = Solution::rob_tdown(nums);
        assert_eq!(result1, 4);
        assert_eq!(result2, 4);
    }

    #[test]
    fn case_02() {
        let nums = vec![2, 7, 9, 3, 1];
        let result1 = Solution::rob_bup(nums.clone());
        let result2 = Solution::rob_tdown(nums);
        assert_eq!(result1, 12);
        assert_eq!(result2, 12);
    }
}
