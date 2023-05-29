use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut map: HashMap<i32, i32> = HashMap::new();
        let mut prefix = 0;
        let mut ans = 0;
        map.insert(0, 1);

        for i in 0..nums.len() {
            prefix += nums[i];

            if let Some(value) = map.get(&(prefix - k)) {
                ans += value;
            }

            let entry = map.entry(prefix).or_insert(0);
            *entry += 1;
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let nums = vec![1, 1, 1];
        let result = Solution::subarray_sum(nums, 2);
        assert_eq!(result, 2);
    }

    #[test]
    fn case_02() {
        let nums = vec![1, 2, 3];
        let result = Solution::subarray_sum(nums, 3);
        assert_eq!(result, 2);
    }

    #[test]
    fn case_03() {
        let nums = vec![1];
        let result = Solution::subarray_sum(nums, 3);
        assert_eq!(result, 0);
    }

    #[test]
    fn case_04() {
        let nums = vec![-1, -1, 1];
        let result = Solution::subarray_sum(nums, 3);
        assert_eq!(result, 0);
    }

    #[test]
    fn case_05() {
        let nums = vec![-2, 1, 2, -2, 5, -2, 1];
        let result = Solution::subarray_sum(nums, 3);
        assert_eq!(result, 5);
    }

    #[test]
    fn case_06() {
        let nums = vec![1];
        let result = Solution::subarray_sum(nums, 0);
        assert_eq!(result, 0);
    }
}
