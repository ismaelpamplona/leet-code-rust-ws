use std::collections::HashMap;
struct Solution;
impl Solution {
    pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
        let mut map: HashMap<i32, i32> = HashMap::new();
        map.insert(0, 0);
        let mut sum = 0;

        for right_index in 0..nums.len() {
            sum += nums[right_index];
            if let Some(&left_index) = map.get(&((sum % k + k) % k)) {
                if left_index < right_index as i32 {
                    return true;
                }
            } else {
                map.insert((sum % k + k) % k, right_index as i32 + 1);
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let nums = vec![23, 2, 4, 6, 7];
        let result1 = Solution::check_subarray_sum(nums, 6);
        assert_eq!(result1, true);
    }
    // Explanation: [2, 4] is a continuous subarray of size 2 whose elements sum up to 6.

    #[test]
    fn case_02() {
        let nums = vec![23, 2, 6, 4, 7];
        let result1 = Solution::check_subarray_sum(nums, 6);
        assert_eq!(result1, true);
    }
    // Explanation: [23, 2, 6, 4, 7] is an continuous subarray of size 5 whose elements sum up to 42. 42 is a multiple of 6 because 42 = 7 * 6 and 7 is an integer.

    #[test]
    fn case_03() {
        let nums = vec![23, 2, 6, 4, 7];
        let result1 = Solution::check_subarray_sum(nums, 13);
        assert_eq!(result1, false);
    }

    #[test]
    fn case_04() {
        let nums = vec![2, 4, 3];
        let result1 = Solution::check_subarray_sum(nums, 6);
        assert_eq!(result1, true);
    }

    #[test]
    fn case_05() {
        let nums = vec![1, 3, 6, 0, 9, 6, 9];
        let result1 = Solution::check_subarray_sum(nums, 7);
        assert_eq!(result1, true);
    }
}
