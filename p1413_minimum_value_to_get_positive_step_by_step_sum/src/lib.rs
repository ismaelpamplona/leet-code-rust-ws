use std::cmp;

struct Solution;
impl Solution {
    pub fn min_start_value(nums: Vec<i32>) -> i32 {
        let mut min_value = 0;
        let mut sum = 0;
        for i in 0..nums.len() {
            sum += nums[i];
            min_value = cmp::min(min_value, sum);
        }
        -min_value + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let nums = vec![-3, 2, -3, 4, 2];
        let result = Solution::min_start_value(nums);
        assert_eq!(result, 5);
    }

    #[test]
    fn case_02() {
        let nums = vec![1, 2];
        let result = Solution::min_start_value(nums);
        assert_eq!(result, 1);
    }

    #[test]
    fn case_03() {
        let nums = vec![1, -2, -3];
        let result = Solution::min_start_value(nums);
        assert_eq!(result, 5);
    }

    #[test]
    fn case_04() {
        let nums = vec![2, 3, 5, -5, -1];
        let result = Solution::min_start_value(nums);
        assert_eq!(result, 1);
    }
}
