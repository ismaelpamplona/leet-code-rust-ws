struct Solution;
impl Solution {
    pub fn ways_to_split_array(nums: Vec<i32>) -> i32 {
        let mut prefix: Vec<i64> = vec![nums[0] as i64];
        for i in 1..nums.len() {
            let sum = (nums[i] as i64) + prefix[i - 1];
            prefix.push(sum);
        }

        let mut ans = 0;
        let last = prefix.len() - 1;
        for i in 0..(prefix.len() - 1) {
            if prefix[i] >= (prefix[last] - prefix[i]) {
                ans += 1;
            }
        }
        ans

        // time complexity: O(n)
        // space complexity: O(n)
    }

    pub fn ways_to_split_array_2(nums: Vec<i32>) -> i32 {
        let mut total: i64 = nums[0] as i64;
        for i in 1..nums.len() {
            total += nums[i] as i64;
        }

        let mut ans = 0;
        let mut left: i64 = 0;
        for i in 0..nums.len() - 1 {
            left += nums[i] as i64;
            let right = total - left;
            if left >= right {
                ans += 1;
            }
        }
        ans

        // time complexity: O(n)
        // space complexity: O(1)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let nums = vec![10, 4, -8, 7];
        let result = Solution::ways_to_split_array(nums.clone());
        let result2 = Solution::ways_to_split_array_2(nums.clone());
        assert_eq!(result, 2);
        assert_eq!(result2, 2);
    }

    #[test]
    fn case_02() {
        let nums = vec![2, 3, 1, 0];
        let result = Solution::ways_to_split_array(nums.clone());
        let result2 = Solution::ways_to_split_array_2(nums.clone());
        assert_eq!(result, 2);
        assert_eq!(result2, 2);
    }

    #[test]
    fn case_03() {
        let nums = vec![0, 0];
        let result = Solution::ways_to_split_array(nums.clone());
        let result2 = Solution::ways_to_split_array_2(nums.clone());
        assert_eq!(result, 1);
        assert_eq!(result2, 1);
    }

    #[test]
    fn case_04() {
        let nums = vec![0, -1, -2, -3, -4];
        let result = Solution::ways_to_split_array(nums.clone());
        let result2 = Solution::ways_to_split_array_2(nums.clone());
        assert_eq!(result, 3);
        assert_eq!(result2, 3);
    }
}
