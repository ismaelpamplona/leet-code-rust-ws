use std::{cmp::Reverse, collections::BinaryHeap};

#[derive(Debug)]
struct MedianFinder {
    min_heap: BinaryHeap<Reverse<i32>>,
    max_heap: BinaryHeap<i32>,
}

impl MedianFinder {
    fn new() -> Self {
        MedianFinder {
            min_heap: BinaryHeap::new(),
            max_heap: BinaryHeap::new(),
        }
    }

    fn add_num(&mut self, num: i32) {
        self.max_heap.push(num);
        self.min_heap.push(Reverse(self.max_heap.pop().unwrap()));
        if self.min_heap.len() > self.max_heap.len() {
            self.max_heap.push(self.min_heap.pop().unwrap().0);
        }
    }

    fn from(nums: Vec<i32>) -> Self {
        let mut median = Self::new();
        for num in nums {
            median.add_num(num);
        }
        median
    }

    fn find_median(&self) -> f64 {
        if self.max_heap.len() > self.min_heap.len() {
            return *self.max_heap.peek().unwrap() as f64;
        }
        return (*self.max_heap.peek().unwrap() as f64 + (*self.min_heap.peek().unwrap()).0 as f64)
            / 2.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let nums = vec![1, 2];
        let mut median_finder = MedianFinder::from(nums);
        println!("{:?}", median_finder);
        assert_eq!(median_finder.find_median(), 1.5);
        median_finder.add_num(3);
        println!("{:?}", median_finder);
        assert_eq!(median_finder.find_median(), 2.0);
    }
}
