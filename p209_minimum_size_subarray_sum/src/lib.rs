struct Solution;
impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut min_length = (nums.len() + 1) as i32;
        let mut sum: i32 = 0;
        let mut left = 0;

        for right in 0..nums.len() {
            sum += nums[right];

            while sum >= target {
                min_length = min_length.min((right - left + 1) as i32);
                sum -= nums[left];
                left += 1;
            }
        }

        if min_length == (nums.len() + 1) as i32 {
            return 0;
        }

        min_length
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let nums = vec![2, 3, 1, 2, 4, 3];
        let result = Solution::min_sub_array_len(7, nums);
        // assert_eq!(result, 2);
    }

    #[test]
    fn case_02() {
        let nums = vec![1, 4, 4];
        let result = Solution::min_sub_array_len(4, nums);
        // assert_eq!(result, 1);
    }

    #[test]
    fn case_03() {
        let nums = vec![1, 1, 1, 1, 1, 1, 1, 1];
        let result = Solution::min_sub_array_len(11, nums);
        // assert_eq!(result, 0);
    }
}
