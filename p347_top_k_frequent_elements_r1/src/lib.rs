use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

#[allow(dead_code)]
struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn top_k_frequent_a(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        for n in nums {
            map.entry(n).and_modify(|entry| *entry += 1).or_insert(0);
        }
        let mut min_heap = BinaryHeap::new();
        for (a, b) in map {
            min_heap.push(Reverse((b, a)));
            if min_heap.len() > k as usize {
                min_heap.pop();
            }
        }
        min_heap
            .into_iter()
            .map(|Reverse((v, k))| k)
            .collect::<Vec<i32>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let nums = vec![1, 1, 1, 2, 2, 3];
        let result1 = Solution::top_k_frequent_a(nums.clone(), 2);
        let expected1 = vec![2, 1];
        assert_eq!(result1, expected1);
    }

    #[test]
    fn case_02() {
        let nums = vec![1];
        let result1 = Solution::top_k_frequent_a(nums.clone(), 1);
        let expected = vec![1];
        assert_eq!(result1, expected);
    }
}
