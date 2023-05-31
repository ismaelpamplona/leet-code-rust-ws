use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn number_of_subarrays_count(nums: Vec<i32>, k: i32) -> i32 {
        let mut map: HashMap<i32, i32> = HashMap::new();
        map.insert(0, 1);
        let mut ans = 0;
        let mut cur_prefix_count = 0;
        for i in 0..nums.len() {
            cur_prefix_count += nums[i] % 2;
            if let Some(value) = map.get(&(cur_prefix_count - k)) {
                ans += value;
            }
            let entry = map.entry(cur_prefix_count).or_insert(0);
            *entry += 1;
        }
        ans
    }

    pub fn number_of_subarrays_sw(nums: Vec<i32>, k: i32) -> i32 {
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
        let result1 = Solution::number_of_subarrays_count(nums.clone(), 3);
        let result2 = Solution::number_of_subarrays_sw(nums.clone(), 3);
        assert_eq!(result1, 2);
        assert_eq!(result2, 2);
    }

    #[test]
    fn case_02() {
        let nums = vec![2, 4, 6];
        let result1 = Solution::number_of_subarrays_count(nums.clone(), 1);
        let result2 = Solution::number_of_subarrays_sw(nums.clone(), 1);
        assert_eq!(result1, 0);
        assert_eq!(result2, 0);
    }

    #[test]
    fn case_03() {
        let nums = vec![2, 2, 2, 1, 2, 2, 1, 2, 2, 2];
        let result1 = Solution::number_of_subarrays_count(nums.clone(), 2);
        let result2 = Solution::number_of_subarrays_sw(nums.clone(), 2);
        assert_eq!(result1, 16);
        assert_eq!(result2, 16);
    }
}
