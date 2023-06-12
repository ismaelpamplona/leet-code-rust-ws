use std::collections::HashSet;

struct Solution;
impl Solution {
    pub fn maximum_unique_subarray(nums: Vec<i32>) -> i32 {
        let (mut ans, mut l) = (0, 0);
        let mut set: HashSet<i32> = HashSet::new();
        let mut cur_sum = 0;

        for r in 0..nums.len() {
            while set.contains(&nums[r]) && l < r {
                set.remove(&nums[l]);
                cur_sum -= nums[l];
                l += 1;
            }
            cur_sum += nums[r];
            set.insert(nums[r]);

            ans = ans.max(cur_sum);
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let nums = vec![4, 2, 4, 5, 6];
        let result = Solution::maximum_unique_subarray(nums);
        assert_eq!(result, 17);
    }

    #[test]
    fn case_02() {
        let nums = vec![5, 2, 1, 2, 5, 2, 1, 2, 5];
        let result = Solution::maximum_unique_subarray(nums);
        assert_eq!(result, 8);
    }
}
