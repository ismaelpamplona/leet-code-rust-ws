struct Solution;
impl Solution {
    pub fn number_of_subarrays(nums: Vec<i32>, k: i32) -> i32 {
        let mut left = 0;
        let mut ans = 0;
        let mut sum_odds = 0;
        let mut count = 0;

        for right in 0..nums.len() {
            if nums[right] % 2 != 0 {
                sum_odds += 1;
                count = 0;
            }

            while sum_odds == k {
                if nums[left] % 2 != 0 {
                    sum_odds -= 1;
                }
                left += 1;
                count += 1;
            }

            ans += count;
        }

        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let nums = vec![1, 1, 2, 1, 1];
        let result = Solution::number_of_subarrays(nums, 3);
        assert_eq!(result, 2);
    }

    #[test]
    fn case_02() {
        let nums = vec![2, 4, 6];
        let result = Solution::number_of_subarrays(nums, 1);
        assert_eq!(result, 0);
    }

    #[test]
    fn case_03() {
        let nums = vec![2, 2, 2, 1, 2, 2, 1, 2, 2, 2];
        let result = Solution::number_of_subarrays(nums, 2);
        assert_eq!(result, 16);
    }
}
