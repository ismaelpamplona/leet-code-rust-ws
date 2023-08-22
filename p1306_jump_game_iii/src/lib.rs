use std::collections::{HashMap, HashSet, VecDeque};

struct Solution;
impl Solution {
    pub fn can_reach(arr: Vec<i32>, start: i32) -> bool {
        fn get_neighbours(i: usize, arr: &Vec<i32>) -> Vec<usize> {
            let before = i as i32 - arr[i];
            let after = i + arr[i] as usize;
            let mut neighbours = vec![];
            if before >= 0 {
                neighbours.push(before as usize);
            }
            if after < arr.len() {
                neighbours.push(after);
            }
            neighbours
        }
        let mut q = VecDeque::from([start as usize]);
        let mut seen = HashSet::from([start as usize]);
        while let Some(n) = q.pop_front() {
            if arr[n] == 0 {
                return true;
            }
            for neighbour in get_neighbours(n, &arr) {
                if !seen.contains(&neighbour) {
                    q.push_back(neighbour);
                    seen.insert(neighbour);
                }
            }
        }
        false
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let arr = vec![4, 2, 3, 0, 3, 1, 2];
        let start = 5;
        let result1 = Solution::can_reach(arr, start);
        assert_eq!(result1, true);
    }

    #[test]
    fn case_02() {
        let arr = vec![4, 2, 3, 0, 3, 1, 2];
        let start = 0;
        let result1 = Solution::can_reach(arr, start);
        assert_eq!(result1, true);
    }

    #[test]
    fn case_03() {
        let arr = vec![3, 0, 2, 1, 2];
        let start = 2;
        let result1 = Solution::can_reach(arr, start);
        assert_eq!(result1, false);
    }
}
