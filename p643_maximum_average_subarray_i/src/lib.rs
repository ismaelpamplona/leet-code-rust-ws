struct Solution;
impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let (mut sum, mut avg, mut left) = (0, 0.0, 0);

        // build very first sliding window
        for i in 0..k {
            sum += nums[i as usize];
        }

        avg = sum as f64 / k as f64;

        for right in k..nums.len() as i32 {
            sum += nums[right as usize] - nums[(right - k) as usize];
            left += 1;
            let curr = sum as f64 / k as f64;
            avg = avg.max(curr);
        }
        avg
    }

    pub fn find_max_average_2(nums: Vec<i32>, k: i32) -> f64 {
        let (mut sum, mut left) = (0, 0);
        let mut avg: f64 = 0.0;

        for right in 0..nums.len() {
            sum += nums[right];

            if right >= k as usize {
                sum -= nums[left];
                left += 1;
            }
            let curr = sum as f64 / k as f64;
            avg = avg.max(curr);
        }

        avg
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let vec = vec![1, 12, -5, -6, 50, 3];
        let result = Solution::find_max_average(vec.clone(), 4);
        let result_2 = Solution::find_max_average_2(vec, 4);
        assert_eq!(result, 12.75000);
        assert_eq!(result_2, 12.75000);
    }

    #[test]
    fn case_02() {
        let vec = vec![5];
        let result = Solution::find_max_average(vec.clone(), 1);
        let result_2 = Solution::find_max_average_2(vec, 1);
        assert_eq!(result, 5.00000);
        assert_eq!(result_2, 5.00000);
    }
}
