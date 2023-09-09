#[allow(dead_code)]
struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn smallest_divisor(nums: Vec<i32>, threshold: i32) -> i32 {
        fn check(nums: &Vec<i32>, divisor: f64, threshold: f64) -> bool {
            let mut sum = 0.0;
            for num in nums {
                sum += (*num as f64 / divisor).ceil()
            }
            sum <= threshold
        }
        let mut left = 0;
        let mut right = *nums.iter().max().unwrap();
        while left <= right {
            let mid = (left + right) / 2;
            if check(&nums, mid as f64, threshold as f64) {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }
        left
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let nums = vec![1, 2, 5, 9];
        let result1 = Solution::smallest_divisor(nums, 6);
        assert_eq!(result1, 5);
    }

    #[test]
    fn case_02() {
        let nums = vec![44, 22, 33, 11, 1];
        let result1 = Solution::smallest_divisor(nums, 5);
        assert_eq!(result1, 44);
    }
}
