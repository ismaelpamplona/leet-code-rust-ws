use std::collections::HashMap;

struct Solution;
impl Solution {
    fn dp(i: usize, nums: &[i32], cache: &mut HashMap<usize, i32>) -> i32 {
        if let Some(&cached) = cache.get(&i) {
            return cached;
        }

        let mut ans = 1;
        for j in 0..i {
            if nums[i] > nums[j] {
                ans = ans.max(Self::dp(j, nums, cache) + 1);
            }
        }
        cache.insert(i, ans);
        ans
    }
    pub fn length_of_lis_tdown(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut cache = HashMap::new();
        for i in 0..nums.len() {
            ans = ans.max(Self::dp(i, &nums, &mut cache));
        }
        ans
    }

    pub fn length_of_lis_bup(nums: Vec<i32>) -> i32 {
        let mut max = 1;
        let mut cache = vec![1; nums.len()];
        for i in 0..nums.len() {
            for j in 0..i {
                if nums[i] > nums[j] {
                    cache[i] = cache[i].max(cache[j] + 1);
                    max = max.max(cache[i]);
                }
            }
        }
        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let nums = vec![10, 9, 2, 5, 3, 7, 101, 18];
        let result1 = Solution::length_of_lis_tdown(nums.clone());
        let result2 = Solution::length_of_lis_bup(nums.clone());
        assert_eq!(result1, 4);
        assert_eq!(result2, 4);
    }

    #[test]
    fn case_02() {
        let nums = vec![0, 1, 0, 3, 2, 3];
        let result1 = Solution::length_of_lis_tdown(nums.clone());
        let result2 = Solution::length_of_lis_bup(nums.clone());
        assert_eq!(result1, 4);
        assert_eq!(result2, 4);
    }

    #[test]
    fn case_03() {
        let nums = vec![7, 7, 7, 7, 7, 7, 7];
        let result1 = Solution::length_of_lis_tdown(nums.clone());
        let result2 = Solution::length_of_lis_bup(nums.clone());
        assert_eq!(result1, 1);
        assert_eq!(result2, 1);
    }
}
