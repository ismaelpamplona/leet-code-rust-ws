use std::{cmp::Reverse, collections::BTreeMap};

#[allow(dead_code)]
#[derive(Default, Debug)]
struct MultiSet<T> {
    count: usize,
    data: BTreeMap<T, usize>,
}

#[allow(dead_code)]
impl<T: Ord + Copy> MultiSet<T> {
    fn insert(&mut self, val: T) {
        self.data.entry(val).and_modify(|c| *c += 1).or_insert(1);
        self.count += 1;
    }

    fn remove(&mut self, val: &T) -> Option<T> {
        if let Some(count) = self.data.get_mut(val) {
            *count -= 1;
            if *count == 0 {
                self.data.remove(val);
            }
            self.count -= 1;
            return Some(*val);
        }
        None
    }

    fn last(&mut self) -> Option<T> {
        self.data.iter().next_back().map(|(k, _)| *k)
    }

    fn pop_last(&mut self) -> Option<T> {
        self.last().and_then(|l| self.remove(&l))
    }

    fn len(&self) -> usize {
        self.count
    }
}

#[allow(dead_code)]
#[derive(Default, Debug)]
struct MedianFinder {
    lo: MultiSet<i64>,
    hi: MultiSet<Reverse<i64>>,
}

#[allow(dead_code)]
impl MedianFinder {
    fn push(&mut self, val: i64) {
        self.lo.insert(val);
        self.hi.insert(Reverse(self.lo.pop_last().unwrap()));
        if self.lo.len() < self.hi.len() {
            self.lo.insert(self.hi.pop_last().unwrap().0);
        }
    }

    fn pop(&mut self, val: i64) -> Option<i64> {
        if let Some(popped) = self.lo.remove(&val) {
            if self.lo.len() < self.hi.len() {
                self.lo.insert(self.hi.pop_last().unwrap().0);
            }
            return Some(popped);
        }
        if let Some(popped) = self.hi.remove(&Reverse(val)) {
            if self.lo.len() - self.hi.len() > 1 {
                self.hi.insert(Reverse(self.lo.pop_last().unwrap()));
            }
            return Some(popped.0);
        }
        None
    }

    fn find_median(&mut self) -> Option<f64> {
        let max = self.lo.last()?;
        if self.lo.len() != self.hi.len() {
            return Some(max as f64);
        }
        let min = self.hi.last().unwrap().0;
        Some((max + min) as f64 / 2.0)
    }
}

struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn median_sliding_window(nums: Vec<i32>, k: i32) -> Vec<f64> {
        let (n, k) = (nums.len(), k as usize);
        let nums = nums.into_iter().map(|e| e as i64).collect::<Vec<i64>>();
        let mut ans = Vec::with_capacity(n);
        let mut heap = MedianFinder::default();
        for n in nums.iter().take(k) {
            heap.push(*n);
        }
        ans.push(heap.find_median().unwrap());
        for i in k..nums.len() {
            heap.push(nums[i]);
            heap.pop(nums[i - k]);
            ans.push(heap.find_median().unwrap());
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let nums = vec![1, 3, -1, -3, 5, 3, 6, 7];
        let expected = vec![1.00000, -1.00000, -1.00000, 3.00000, 5.00000, 6.00000];
        let result1 = Solution::median_sliding_window(nums.clone(), 3);
        assert_eq!(result1, expected);
    }

    #[test]
    fn case_02() {
        let nums = vec![1, 2, 3, 4, 2, 3, 1, 4, 2];
        let expected = vec![
            2.00000, 3.00000, 3.00000, 3.00000, 2.00000, 3.00000, 2.00000,
        ];
        let result1 = Solution::median_sliding_window(nums.clone(), 3);
        assert_eq!(result1, expected);
    }

    #[test]
    fn case_03() {
        let nums = vec![1];
        let expected = vec![1.00000];
        let result1 = Solution::median_sliding_window(nums.clone(), 1);
        assert_eq!(result1, expected);
    }

    #[test]
    fn case_04() {
        let nums = vec![7, 0, 3, 9, 9, 9, 1, 7, 2, 3];
        let expected = vec![8.00000, 6.00000, 8.00000, 8.00000, 5.00000];
        let result1 = Solution::median_sliding_window(nums.clone(), 6);
        assert_eq!(result1, expected);
    }

    #[test]
    fn case_05() {
        let nums = vec![7, 8, 8, 3, 8, 1, 5, 3, 5, 4];
        let expected = vec![
            8.00000, 8.00000, 8.00000, 3.00000, 5.00000, 3.00000, 5.00000, 4.00000,
        ];
        let result1 = Solution::median_sliding_window(nums.clone(), 3);
        assert_eq!(result1, expected);
    }
}
