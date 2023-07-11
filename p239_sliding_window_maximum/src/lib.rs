use std::collections::VecDeque;

struct Solution;
impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut ans: Vec<i32> = vec![];
        let mut q: VecDeque<usize> = VecDeque::new();
        for i in 0..nums.len() {
            while !q.is_empty() && nums[i] > nums[q[q.len() - 1]] {
                q.pop_back();
            }
            q.push_back(i);

            if q[0] + k as usize == i {
                q.pop_front();
            }

            if i >= k as usize - 1 {
                ans.push(nums[q[0]] as i32)
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let nums = vec![1, 3, -1, -3, 5, 3, 6, 7];
        let k = 3;
        let output = vec![3, 3, 5, 5, 6, 7];
        let result = Solution::max_sliding_window(nums, k);
        assert_eq!(result, output)
    }

    #[test]
    fn case_02() {
        let nums = vec![1];
        let k = 1;
        let output = vec![1];
        let result = Solution::max_sliding_window(nums, k);
        assert_eq!(result, output)
    }
}
