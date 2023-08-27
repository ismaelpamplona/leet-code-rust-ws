use std::{cmp::Reverse, collections::BinaryHeap};

struct Solution;
impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut heap = BinaryHeap::from(nums);
        let mut ans = 0;
        let mut count = 0;
        while count < k {
            ans = heap.pop().unwrap();
            count += 1;
        }
        ans
    }

    pub fn find_kth_largest_min(nums: Vec<i32>, k: i32) -> i32 {
        let mut heap = BinaryHeap::with_capacity(nums.len());
        for num in nums {
            heap.push(Reverse(num));
            if heap.len() > k as usize {
                heap.pop();
            }
        }
        heap.peek().unwrap().0
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let nums = vec![3, 2, 1, 5, 6, 4];
        let result1 = Solution::find_kth_largest(nums.clone(), 2);
        let result2 = Solution::find_kth_largest_min(nums, 2);
        assert_eq!(result1, 5);
        assert_eq!(result2, 5);
    }

    #[test]
    fn case_02() {
        let nums = vec![3, 2, 3, 1, 2, 4, 5, 5, 6];
        let result1 = Solution::find_kth_largest(nums.clone(), 4);
        let result2 = Solution::find_kth_largest_min(nums, 4);
        assert_eq!(result1, 4);
        assert_eq!(result2, 4);
    }
}
