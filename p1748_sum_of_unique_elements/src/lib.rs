use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn sum_of_unique(nums: Vec<i32>) -> i32 {
        let mut map: HashMap<i32, i32> = HashMap::new();
        let mut ans = 0;

        for i in 0..nums.len() {
            let entry = map.entry(nums[i]).or_insert(0);
            *entry += 1;
        }

        for i in 0..nums.len() {
            if let Some(value) = map.get(&nums[i]) {
                if *value == 1 {
                    ans += nums[i];
                }
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
        let nums = vec![1, 2, 3, 2];
        let result = Solution::sum_of_unique(nums);
        assert_eq!(result, 4);
    }

    #[test]
    fn case_02() {
        let nums = vec![1, 1, 1, 1, 1];
        let result = Solution::sum_of_unique(nums);
        assert_eq!(result, 0);
    }

    #[test]
    fn case_03() {
        let nums = vec![1, 2, 3, 4, 5];
        let result = Solution::sum_of_unique(nums);
        assert_eq!(result, 15);
    }

    #[test]
    fn case_04() {
        let nums = vec![1, 2, 3, 4, 5, 1];
        let result = Solution::sum_of_unique(nums);
        assert_eq!(result, 14);
    }
}
