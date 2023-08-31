#[allow(dead_code)]
struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0 as i64;
        let mut right = nums.len() as i64 - 1;
        while left <= right {
            let mid = left + (right - left) / 2;
            if nums[mid as usize] == target {
                return mid as i32;
            } else if nums[mid as usize] > target {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }
        left as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let nums = vec![1, 3, 5, 6];
        let target = 5;
        let result1 = Solution::search_insert(nums.clone(), target);
        assert_eq!(result1, 2);
    }

    #[test]
    fn case_02() {
        let nums = vec![1, 3, 5, 6];
        let target = 2;
        let result1 = Solution::search_insert(nums.clone(), target);
        assert_eq!(result1, 1);
    }

    #[test]
    fn case_03() {
        let nums = vec![1, 3, 5, 6];
        let target = 7;
        let result1 = Solution::search_insert(nums.clone(), target);
        assert_eq!(result1, 4);
    }
}
