use std::collections::HashSet;

struct Solution;
impl Solution {
    pub fn maximum_unique_subarray_set(nums: Vec<i32>) -> i32 {
        let (mut ans, mut l, mut cur_sum) = (0, 0, 0);
        let mut set: HashSet<i32> = HashSet::new();

        for r in 0..nums.len() {
            while set.contains(&nums[r]) && l < r {
                set.remove(&nums[l]);
                cur_sum -= nums[l];
                l += 1;
            }
            cur_sum += nums[r];
            set.insert(nums[r]);

            ans = ans.max(cur_sum);
        }

        ans
    }

    pub fn maximum_unique_subarray_bool(nums: Vec<i32>) -> i32 {
        let (mut ans, mut l, mut cur_sum) = (0, 0, 0);
        let mut is_present: Vec<bool> = vec![false; 10001];

        for r in 0..nums.len() {
            while is_present[nums[r] as usize] {
                is_present[nums[l] as usize] = false;
                cur_sum -= nums[l];
                l += 1;
            }
            is_present[nums[r] as usize] = true;
            cur_sum += nums[r];

            ans = ans.max(cur_sum);
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let nums = vec![4, 2, 4, 5, 6];
        let result1 = Solution::maximum_unique_subarray_set(nums.clone());
        let result2 = Solution::maximum_unique_subarray_bool(nums.clone());
        assert_eq!(result1, 17);
        assert_eq!(result2, 17);
    }

    #[test]
    fn case_02() {
        let nums = vec![5, 2, 1, 2, 5, 2, 1, 2, 5];
        let result1 = Solution::maximum_unique_subarray_set(nums.clone());
        let result2 = Solution::maximum_unique_subarray_bool(nums.clone());
        assert_eq!(result1, 8);
        assert_eq!(result2, 8);
    }
}
