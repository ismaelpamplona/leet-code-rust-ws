use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        let mut map: HashMap<i32, i32> = HashMap::new();
        let mut ans = 0;

        for i in 0..nums.len() {
            if let Some(value) = map.get(&nums[i]) {
                ans += *value;
                map.insert(nums[i], value + 1);
            } else {
                map.insert(nums[i], 1);
            }
        }

        ans as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let nums = vec![1, 2, 3, 1, 1, 3];
        let result = Solution::num_identical_pairs(nums);
        assert_eq!(result, 4);
    }

    #[test]
    fn case_02() {
        let nums = vec![1, 1, 1, 1];
        let result = Solution::num_identical_pairs(nums);
        assert_eq!(result, 6);
    }

    #[test]
    fn case_03() {
        let nums = vec![1, 2, 3];
        let result = Solution::num_identical_pairs(nums);
        assert_eq!(result, 0);
    }

    #[test]
    fn case_04() {
        let nums = vec![
            6, 5, 1, 5, 7, 7, 9, 1, 5, 7, 1, 6, 10, 9, 7, 4, 1, 8, 7, 1, 1, 8, 6, 4, 7, 4, 10, 5,
            3, 9, 10, 1, 9, 5, 5, 4, 1, 7, 4, 2, 9, 2, 6, 6, 4, 2, 10, 3, 5, 3, 6, 4, 7, 4, 6, 4,
            4, 6, 3, 4, 10, 1, 10, 6, 10, 4, 9, 6, 6, 4, 8, 6, 9, 5, 4,
        ];
        let result = Solution::num_identical_pairs(nums);
        assert_eq!(result, 303);
    }
}
