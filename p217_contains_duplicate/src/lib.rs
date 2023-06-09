use std::collections::HashSet;
struct Solution;
impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut set: HashSet<i32> = HashSet::new();
        for i in 0..nums.len() {
            if set.contains(&nums[i]) {
                return true;
            } else {
                set.insert(nums[i]);
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
        let nums = vec![1, 2, 3, 1];
        let result1 = Solution::contains_duplicate(nums);
        assert_eq!(result1, true);
    }

    #[test]
    fn case_02() {
        let nums = vec![1, 2, 3, 4];
        let result1 = Solution::contains_duplicate(nums);
        assert_eq!(result1, false);
    }

    #[test]
    fn case_03() {
        let nums = vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2];
        let result1 = Solution::contains_duplicate(nums);
        assert_eq!(result1, true);
    }
}
