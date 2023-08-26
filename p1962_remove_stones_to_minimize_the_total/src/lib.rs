use std::collections::BinaryHeap;

#[allow(dead_code)]
struct Solution;
impl Solution {
    pub fn min_stone_sum(piles: Vec<i32>, k: i32) -> i32 {
        let mut heap = BinaryHeap::from(piles);
        let mut count = 0;
        while count < k {
            if let Some(max) = heap.pop() {
                heap.push(max - max / 2);
            }
            count += 1;
        }
        heap.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let piles = vec![5, 4, 9];
        let result1 = Solution::min_stone_sum(piles, 2);
        assert_eq!(result1, 12);
    }

    #[test]
    fn case_02() {
        let piles = vec![4, 3, 6, 7];
        let result1 = Solution::min_stone_sum(piles, 3);
        assert_eq!(result1, 12);
    }
}
