use std::collections::BinaryHeap;

#[allow(dead_code)]
struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let mut heap = BinaryHeap::with_capacity(arr.len());
        for num in arr {
            let diff = (num - x).abs();
            heap.push((diff, num));
            if heap.len() > k as usize {
                heap.pop();
            }
        }
        let mut result = vec![];
        while let Some((_, n)) = heap.pop() {
            result.push(n);
        }
        result.sort();
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let arr = vec![1, 2, 3, 4, 5];
        let result1 = Solution::find_closest_elements(arr, 4, 3);
        let expected = vec![1, 2, 3, 4];
        assert_eq!(result1, expected);
    }

    #[test]
    fn case_02() {
        let arr = vec![1, 2, 3, 4, 5];
        let result1 = Solution::find_closest_elements(arr, 4, -1);
        let expected = vec![1, 2, 3, 4];
        assert_eq!(result1, expected);
    }
}
