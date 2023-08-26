use std::{cmp::Reverse, collections::BinaryHeap};

struct Solution;
impl Solution {
    pub fn connect_sticks(sticks: Vec<i32>) -> i32 {
        let mut heap = BinaryHeap::from(sticks.iter().map(|&x| Reverse(x)).collect::<Vec<_>>());
        let mut total_cost = 0;
        while let (Some(Reverse(a)), Some(Reverse(b))) = (heap.pop(), heap.pop()) {
            total_cost += a + b;
            heap.push(Reverse(a + b));
        }
        total_cost
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let sticks = vec![2, 4, 3];
        let result1 = Solution::connect_sticks(sticks);
        assert_eq!(result1, 14);
    }

    #[test]
    fn case_02() {
        let sticks = vec![1, 8, 3, 5];
        let result1 = Solution::connect_sticks(sticks);
        assert_eq!(result1, 30);
    }

    #[test]
    fn case_03() {
        let sticks = vec![5];
        let result1 = Solution::connect_sticks(sticks);
        assert_eq!(result1, 0);
    }
}
