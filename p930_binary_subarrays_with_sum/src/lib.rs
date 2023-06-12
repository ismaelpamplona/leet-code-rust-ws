use std::collections::HashMap;
struct Solution;
impl Solution {
    pub fn num_subarrays_with_sum(nums: Vec<i32>, goal: i32) -> i32 {
        let mut prefix = vec![0; nums.len() + 1];
        let mut map: HashMap<i32, i32> = HashMap::new();
        let mut ans = 0;

        for i in 0..nums.len() {
            prefix[i + 1] = prefix[i] + nums[i];
        }

        for s in &prefix {
            ans += map.get(&(s)).unwrap_or(&0);
            *map.entry(*s + goal).or_insert(0) += 1;
        }

        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let nums = vec![1, 0, 1, 0, 1];
        let result = Solution::num_subarrays_with_sum(nums, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn case_02() {
        let nums = vec![0, 0, 0, 0, 0];
        let result = Solution::num_subarrays_with_sum(nums, 0);
        assert_eq!(result, 15);
    }
}
