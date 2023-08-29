struct Solution;
impl Solution {
    pub fn partition_array(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort();
        let mut first = nums[0];
        let mut count = 1;
        for i in 1..nums.len() {
            if nums[i] - first > k {
                first = nums[i];
                count += 1;
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let nums = vec![3, 6, 1, 2, 5];
        let result1 = Solution::partition_array(nums.clone(), 2);
        assert_eq!(result1, 2);
    }

    #[test]
    fn case_02() {
        let nums = vec![1, 2, 3];
        let result1 = Solution::partition_array(nums.clone(), 2);
        assert_eq!(result1, 1);
    }

    #[test]
    fn case_03() {
        let nums = vec![2, 2, 4, 5];
        let result1 = Solution::partition_array(nums.clone(), 0);
        assert_eq!(result1, 3);
    }
}
