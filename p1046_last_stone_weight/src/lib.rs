use std::collections::BinaryHeap;

struct Solution;
impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        let mut heap = BinaryHeap::from(stones);
        let mut last = 0;
        while let Some(y) = heap.pop() {
            last = y;
            if let Some(x) = heap.pop() {
                if y != x {
                    heap.push(y - x);
                } else if x == y {
                    last = 0;
                }
            }
        }
        last
    }

    pub fn last_stone_weight_2(stones: Vec<i32>) -> i32 {
        let mut heap = BinaryHeap::from(stones);
        while let Some(y) = heap.pop() {
            match (y, heap.pop()) {
                (y, Some(x)) if y > x => heap.push(y - x),
                (y, None) => return y,
                _ => (),
            }
        }
        0
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let stones = vec![2, 7, 4, 1, 8, 1];
        let result1 = Solution::last_stone_weight(stones.clone());
        let result2 = Solution::last_stone_weight_2(stones);
        assert_eq!(result1, 1);
        assert_eq!(result2, 1);
    }

    #[test]
    fn case_02() {
        let stones = vec![1];
        let result1 = Solution::last_stone_weight(stones.clone());
        let result2 = Solution::last_stone_weight_2(stones);
        assert_eq!(result1, 1);
        assert_eq!(result2, 1);
    }

    #[test]
    fn case_03() {
        let stones = vec![2, 2];
        let result1 = Solution::last_stone_weight(stones.clone());
        let result2 = Solution::last_stone_weight_2(stones);
        assert_eq!(result1, 0);
        assert_eq!(result2, 0);
    }
}
