use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn largest_unique_number(nums: Vec<i32>) -> i32 {
        let mut ans = -1;
        let mut map: HashMap<i32, i32> = HashMap::new();

        for i in 0..nums.len() {
            let entry = map.entry(nums[i]).or_insert(0);
            *entry += 1;
        }

        for (key, val) in map {
            if val == 1 {
                ans = ans.max(key);
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
        let nums = vec![5, 7, 3, 9, 4, 9, 8, 3, 1];
        let result = Solution::largest_unique_number(nums);
        assert_eq!(result, 8);
    }

    #[test]
    fn case_02() {
        let nums = vec![9, 9, 8, 8];
        let result = Solution::largest_unique_number(nums);
        assert_eq!(result, -1);
    }
}
