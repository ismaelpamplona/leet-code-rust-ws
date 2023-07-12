use std::collections::{HashMap, VecDeque};

struct Solution;
impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();
        let mut stack: VecDeque<i32> = VecDeque::new();
        for i in 0..nums2.len() {
            while !stack.is_empty() && nums2[i] > stack[stack.len() - 1] {
                map.insert(stack.pop_back().unwrap(), nums2[i]);
            }
            stack.push_back(nums2[i]);
        }
        while !stack.is_empty() {
            map.insert(stack.pop_back().unwrap(), -1);
        }

        let mut ans: Vec<i32> = vec![0; nums1.len()];
        for i in 0..nums1.len() {
            if let Some(v) = map.get(&nums1[i]) {
                ans[i] = *v;
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
        let nums1 = vec![4, 1, 2];
        let nums2 = vec![1, 3, 4, 2];
        let output = vec![-1, 3, -1];
        let result1 = Solution::next_greater_element(nums1.clone(), nums2.clone());
        assert_eq!(result1, output);
    }

    #[test]
    fn case_02() {
        let nums1 = vec![2, 4];
        let nums2 = vec![1, 2, 3, 4];
        let output = vec![3, -1];
        let result1 = Solution::next_greater_element(nums1.clone(), nums2.clone());
        assert_eq!(result1, output);
    }
}
