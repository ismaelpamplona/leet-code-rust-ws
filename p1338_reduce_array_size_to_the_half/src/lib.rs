use std::collections::{BinaryHeap, HashMap};

struct Solution;
impl Solution {
    pub fn min_set_size(arr: Vec<i32>) -> i32 {
        let mut map = HashMap::new();
        for n in &arr {
            *map.entry(n).or_insert(0) += 1;
        }
        let mut heap = BinaryHeap::new();
        for (k, v) in map {
            heap.push(v);
        }
        let half = arr.len() / 2;
        let mut removed = 0;
        let mut ans = 0;
        while let Some(count) = heap.pop() {
            removed += count;
            ans += 1;
            if removed >= half {
                break;
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
        let arr = vec![3, 3, 3, 3, 5, 5, 5, 2, 2, 7];
        let result1 = Solution::min_set_size(arr);
        assert_eq!(result1, 2);
    }

    #[test]
    fn case_02() {
        let arr = vec![7, 7, 7, 7, 7, 7];
        let result1 = Solution::min_set_size(arr);
        assert_eq!(result1, 1);
    }

    #[test]
    fn case_03() {
        let arr = vec![1, 9];
        let result1 = Solution::min_set_size(arr);
        assert_eq!(result1, 1);
    }
}
