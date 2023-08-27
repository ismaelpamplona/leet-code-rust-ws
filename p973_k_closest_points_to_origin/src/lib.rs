use std::{cmp::Reverse, collections::BinaryHeap};

#[allow(dead_code)]
struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut heap = BinaryHeap::new();
        let mut result = vec![];
        let n = points.len() - k as usize;
        for cord in points {
            let dist = cord[0] * cord[0] + cord[1] * cord[1];
            heap.push(Reverse((dist, cord[0], cord[1])));
            if heap.len() > n {
                let Reverse((_, x, y)) = heap.pop().unwrap();
                result.push(vec![x, y])
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let points = vec![vec![1, 3], vec![-2, 2]];
        let result1 = Solution::k_closest(points, 1);
        let expected = vec![[-2, 2]];
        assert_eq!(result1, expected);
    }

    #[test]
    fn case_02() {
        let points = vec![vec![3, 3], vec![5, -1], vec![-2, 4]];
        let result1 = Solution::k_closest(points, 2);
        let expected = vec![vec![3, 3], vec![-2, 4]];
        assert_eq!(result1, expected);
    }
}
