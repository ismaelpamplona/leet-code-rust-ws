struct Solution;
impl Solution {
    pub fn get_averages_prefix(nums: Vec<i32>, k: i32) -> Vec<i32> {
        if k == 0 {
            return nums;
        }
        let mut result: Vec<i32> = vec![-1; nums.len()];
        if k * 2 + 1 > nums.len() as i32 {
            return result;
        }
        let mut prefix: Vec<i64> = vec![0; nums.len() + 1];
        for i in 0..nums.len() {
            prefix[i + 1] = prefix[i] + nums[i] as i64
        }
        let els = k * 2 + 1;
        for i in ((k * 2) as usize)..nums.len() {
            let right = prefix[i + 1];
            let left = prefix[i - (k as usize * 2)];
            let sum = right - left;
            let avg = sum / els as i64;
            result[i - k as usize] = avg as i32;
        }
        result
    }

    pub fn get_averages_sliding(nums: Vec<i32>, k: i32) -> Vec<i32> {
        if k == 0 {
            return nums;
        }
        let mut result: Vec<i32> = vec![-1; nums.len()];
        if k * 2 + 1 > nums.len() as i32 {
            return result;
        }
        let mut sum: i64 = 0;
        let els = (k * 2 + 1) as i64;
        for i in 0..nums.len() {
            sum += nums[i] as i64;
            let start_pos = i as i32 - (2 * k);
            if start_pos >= 0 {
                if start_pos > 0 {
                    sum -= nums[start_pos as usize - 1] as i64;
                }
                let avg = sum / els;
                result[i - k as usize] = avg as i32;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let nums = vec![7, 4, 3, 9, 1, 8, 5, 2, 6];
        let output = vec![-1, -1, -1, 5, 4, 4, -1, -1, -1];
        let result1 = Solution::get_averages_prefix(nums.clone(), 3);
        let result2 = Solution::get_averages_sliding(nums, 3);
        assert_eq!(result1, output);
        assert_eq!(result2, output);
    }

    #[test]
    fn case_02() {
        let nums = vec![100000];
        let output = vec![100000];
        let result1 = Solution::get_averages_prefix(nums.clone(), 0);
        let result2 = Solution::get_averages_sliding(nums, 0);
        assert_eq!(result1, output);
        assert_eq!(result2, output);
    }

    #[test]
    fn case_03() {
        let nums = vec![8];
        let output = vec![-1];
        let result1 = Solution::get_averages_prefix(nums.clone(), 100000);
        let result2 = Solution::get_averages_sliding(nums, 100000);
        assert_eq!(result1, output);
        assert_eq!(result2, output);
    }
}
