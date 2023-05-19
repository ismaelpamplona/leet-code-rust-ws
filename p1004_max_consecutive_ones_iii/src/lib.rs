struct Solution;
impl Solution {
    pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
        let (mut ans, mut zero_count, mut left) = (0, 0, 0);

        for right in 0..nums.len() {
            ans += 1;
            if nums[right] == 0 {
                zero_count += 1;
            }

            if zero_count > k {
                ans -= 1;
                if nums[left] == 0 {
                    zero_count -= 1;
                }
                left += 1;
            }
        }

        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let nums = vec![1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0];
        let result = Solution::longest_ones(nums.clone(), 2);
        assert_eq!(result, 6);
    }

    #[test]
    fn case_02() {
        let nums = vec![0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1];
        let result = Solution::longest_ones(nums.clone(), 3);
        assert_eq!(result, 10);
    }
}
