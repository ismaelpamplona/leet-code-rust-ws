use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn subarrays_div_by_k(nums: Vec<i32>, k: i32) -> i32 {
        let mut map: HashMap<i32, i32> = HashMap::new();
        map.insert(0, 1);
        let mut ans = 0;
        let mut sum = 0;

        for i in 0..nums.len() {
            sum += nums[i];
            let key = (sum % k + k) % k;
            if let Some(value) = map.get(&key) {
                ans += value;
                map.insert(key, value + 1);
            } else {
                map.insert(key, 1);
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
        let nums = vec![4, 5, 0, -2, -3, 1];
        let result1 = Solution::subarrays_div_by_k(nums, 5);
        assert_eq!(result1, 7);
    }

    // Explanation: There are 7 subarrays with a sum divisible by k = 5: [4, 5, 0, -2, -3, 1], [5], [5, 0], [5, 0, -2, -3], [0], [0, -2, -3], [-2, -3]

    #[test]
    fn case_02() {
        let nums = vec![5];
        let result1 = Solution::subarrays_div_by_k(nums, 9);
        assert_eq!(result1, 0);
    }

    #[test]
    fn case_03() {
        let nums = vec![-1, 2, 9];
        let result1 = Solution::subarrays_div_by_k(nums, 2);
        assert_eq!(result1, 2);
    }
}
