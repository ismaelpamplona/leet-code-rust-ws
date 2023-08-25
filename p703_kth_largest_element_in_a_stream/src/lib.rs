use std::{cmp::Reverse, collections::BinaryHeap};

#[derive(Debug)]
struct KthLargest {
    k: i32,
    heap: BinaryHeap<Reverse<i32>>,
}

impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut heap = BinaryHeap::from(nums.iter().map(|&x| Reverse(x)).collect::<Vec<_>>());
        while heap.len() > k as usize {
            heap.pop();
        }
        KthLargest { k, heap }
    }

    fn add(&mut self, val: i32) -> i32 {
        self.heap.push(Reverse(val));
        self.heap.pop();
        self.heap.peek().unwrap().0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let nums = vec![4, 5, 8, 2];
        let mut kth_largest = KthLargest::new(3, nums);
        assert_eq!(kth_largest.add(3), 4);
        assert_eq!(kth_largest.add(5), 5);
        assert_eq!(kth_largest.add(10), 5);
        assert_eq!(kth_largest.add(9), 8);
        assert_eq!(kth_largest.add(4), 8);
    }
}
