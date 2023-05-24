struct Solution;
impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let mut left_sum = 0;
        let mut total_sum = 0;

        for i in 0..nums.len() {
            total_sum += nums[i]
        }

        for i in 0..nums.len() {
            if left_sum == (total_sum - left_sum - nums[i]) {
                return i as i32;
            }
            left_sum += nums[i];
        }
        -1
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let nums = vec![1, 7, 3, 6, 5, 6];
        let result = Solution::pivot_index(nums);
        assert_eq!(result, 3);
    }

    #[test]
    fn case_02() {
        let nums = vec![1, 2, 3];
        let result = Solution::pivot_index(nums);
        assert_eq!(result, -1);
    }

    #[test]
    fn case_03() {
        let nums = vec![2, 1, -1];
        let result = Solution::pivot_index(nums);
        assert_eq!(result, 0);
    }
}
