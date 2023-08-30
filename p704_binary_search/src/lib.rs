use std::cmp::Ordering::{Equal, Greater, Less};
struct Solution;
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let (mut left, mut right) = (0 as i32, nums.len() as i32 - 1);
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
        -1
    }

    pub fn search_opt(nums: Vec<i32>, target: i32) -> i32 {
        let (mut left, mut right) = (0, nums.len());
        while left < right {
            let mid = left + (right - left) / 2;
            match nums[mid].cmp(&target) {
                Equal => return mid as i32,
                Less => left = mid + 1,
                Greater => right = mid,
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let nums = vec![-1, 0, 3, 5, 9, 12];
        let target = 9;
        let result1 = Solution::search(nums.clone(), target);
        let result2 = Solution::search_opt(nums.clone(), target);
        assert_eq!(result1, 4);
        assert_eq!(result2, 4);
    }

    #[test]
    fn case_02() {
        let nums = vec![-1, 0, 3, 5, 9, 12];
        let target = 2;
        let result1 = Solution::search(nums.clone(), target);
        let result2 = Solution::search_opt(nums.clone(), target);
        assert_eq!(result1, -1);
        assert_eq!(result2, -1);
    }
}
