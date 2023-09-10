struct Solution;
impl Solution {
    pub fn split_array(nums: Vec<i32>, k: i32) -> i32 {
        fn is_valid(max: i32, nums: &Vec<i32>, k: i32) -> bool {
            let mut cur_sum = 0;
            let mut splits = 0;
            for n in nums {
                if cur_sum + n <= max {
                    cur_sum += n;
                } else {
                    cur_sum = *n;
                    splits += 1;
                }
            }
            (splits + 1) <= k
        }
        let mut left = *nums.iter().max().unwrap();
        let mut right = nums.iter().sum();
        let mut min = 0;
        while left <= right {
            let max = (left + right) / 2;
            if is_valid(max, &nums, k) {
                right = max - 1;
                min = max;
            } else {
                left = max + 1;
            }
        }
        min
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let nums = vec![7, 2, 5, 10, 8];
        let result1 = Solution::split_array(nums, 2);
        assert_eq!(result1, 18);
    }

    #[test]
    fn case_02() {
        let nums = vec![1, 2, 3, 4, 5];
        let result1 = Solution::split_array(nums, 2);
        assert_eq!(result1, 9);
    }
}
