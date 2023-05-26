use std::collections::HashSet;
struct Solution;
impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let mut map: HashSet<i32> = HashSet::new();

        for i in 0..nums.len() {
            map.insert(nums[i]);
        }

        for i in 0..=nums.len() {
            if !map.contains(&(i as i32)) {
                return i as i32;
            }
        }

        -1
    }

    pub fn missing_number_xor(nums: Vec<i32>) -> i32 {
        let mut expected_xor = 0;

        for i in 0..=nums.len() {
            expected_xor ^= i as i32;
        }

        let mut real_xor = 0;
        for i in 0..nums.len() {
            real_xor ^= nums[i];
        }

        return expected_xor ^ real_xor;
    }

    pub fn missing_number_sum(nums: Vec<i32>) -> i32 {
        let mut expected_sum = 0;

        for i in 0..=nums.len() {
            expected_sum += i as i32;
        }

        let mut real_sum = 0;
        for i in 0..nums.len() {
            real_sum += nums[i];
        }

        return expected_sum - real_sum;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let nums = vec![3, 0, 1];
        let result1 = Solution::missing_number(nums.clone());
        let result2 = Solution::missing_number_xor(nums.clone());
        let result3 = Solution::missing_number_sum(nums);
        assert_eq!(result1, 2);
        assert_eq!(result2, 2);
        assert_eq!(result3, 2);
    }

    #[test]
    fn case_02() {
        let nums = vec![0, 1];
        let result1 = Solution::missing_number(nums.clone());
        let result2 = Solution::missing_number_xor(nums.clone());
        let result3 = Solution::missing_number_sum(nums);
        assert_eq!(result1, 2);
        assert_eq!(result2, 2);
        assert_eq!(result3, 2);
    }

    #[test]
    fn case_03() {
        let nums = vec![9, 6, 4, 2, 3, 5, 7, 0, 1];
        let result1 = Solution::missing_number(nums.clone());
        let result2 = Solution::missing_number_xor(nums.clone());
        let result3 = Solution::missing_number_sum(nums);
        assert_eq!(result1, 8);
        assert_eq!(result2, 8);
        assert_eq!(result3, 8);
    }

    #[test]
    fn case_04() {
        let nums = vec![0, 1, 2, 3, 4];
        let result1 = Solution::missing_number(nums.clone());
        let result2 = Solution::missing_number_xor(nums.clone());
        let result3 = Solution::missing_number_sum(nums);
        assert_eq!(result1, 5);
        assert_eq!(result2, 5);
        assert_eq!(result3, 5);
    }
}
