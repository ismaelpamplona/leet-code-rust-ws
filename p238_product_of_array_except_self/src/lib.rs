struct Solution;
impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len() - 1;
        let mut ans = vec![0; n + 1];
        let mut left = vec![1; n + 1];
        let mut right = vec![1; n + 1];
        for i in 0..n {
            left[i + 1] = left[i] * nums[i];
            right[n - i - 1] = right[n - i] * nums[n - i];
        }
        for i in 0..=n {
            ans[i] = left[i] * right[i]
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let nums = vec![1, 2, 3, 4];
        let expected = vec![24, 12, 8, 6];
        let result = Solution::product_except_self(nums.clone());
        assert_eq!(expected, result)
    }

    #[test]
    fn case_02() {
        let nums = vec![-1, 1, 0, -3, 3];
        let expected = vec![0, 0, 9, 0, 0];
        let result = Solution::product_except_self(nums.clone());
        assert_eq!(expected, result)
    }

    #[test]
    fn case_03() {
        let nums = vec![4, 5, 1, 8, 2, 10, 6];
        let expected = vec![4800, 3840, 19200, 2400, 9600, 1920, 3200];
        let result = Solution::product_except_self(nums.clone());
        assert_eq!(expected, result)
    }
}
