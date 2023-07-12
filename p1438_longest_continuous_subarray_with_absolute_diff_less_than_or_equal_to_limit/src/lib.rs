use std::collections::VecDeque;

struct Solution;
impl Solution {
    pub fn longest_subarray(nums: Vec<i32>, limit: i32) -> i32 {
        let (mut left, mut ans) = (0, 0);
        let mut increasing: VecDeque<i32> = VecDeque::new();
        let mut decreasing: VecDeque<i32> = VecDeque::new();
        for right in 0..nums.len() {
            while !increasing.is_empty() && increasing[increasing.len() - 1] > nums[right] {
                increasing.pop_back();
            }
            while !decreasing.is_empty() && decreasing[decreasing.len() - 1] < nums[right] {
                decreasing.pop_back();
            }
            increasing.push_back(nums[right]);
            decreasing.push_back(nums[right]);

            while (decreasing[0] - increasing[0]) > limit {
                if nums[left] == decreasing[0] {
                    decreasing.pop_front();
                }
                if nums[left] == increasing[0] {
                    increasing.pop_front();
                }
                left += 1;
            }
            ans = ans.max(right - left + 1);
        }
        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let nums = vec![8, 2, 4, 7];
        let result = Solution::longest_subarray(nums, 4);
        assert_eq!(result, 2);
    }

    #[test]
    fn case_02() {
        let nums = vec![10, 1, 2, 4, 7, 2];
        let result = Solution::longest_subarray(nums, 5);
        assert_eq!(result, 4);
    }

    #[test]
    fn case_03() {
        let nums = vec![4, 2, 2, 2, 4, 4, 2, 2];
        let result = Solution::longest_subarray(nums, 0);
        assert_eq!(result, 3);
    }

    #[test]
    fn case_04() {
        let nums = vec![1, 5, 6, 7, 8, 10, 6, 5, 6];
        let result = Solution::longest_subarray(nums, 4);
        assert_eq!(result, 5);
    }
}
