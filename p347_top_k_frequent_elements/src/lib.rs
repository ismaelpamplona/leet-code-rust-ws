use std::collections::{BinaryHeap, HashMap};

#[allow(dead_code)]
struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        for n in nums {
            map.entry(n).and_modify(|entry| *entry += 1).or_insert(0);
        }
        let mut heap: BinaryHeap<(i32, i32)> =
            map.into_iter().map(|(key, value)| (value, key)).collect();
        let mut result = vec![];
        for _ in 0..k {
            result.push(heap.pop().unwrap().1)
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let nums = vec![1, 1, 1, 2, 2, 3];
        let result1 = Solution::top_k_frequent(nums.clone(), 2);
        let expected = vec![1, 2];
        assert_eq!(result1, expected);
    }

    #[test]
    fn case_02() {
        let nums = vec![1];
        let result1 = Solution::top_k_frequent(nums.clone(), 1);
        let expected = vec![1];
        assert_eq!(result1, expected);
    }
}
