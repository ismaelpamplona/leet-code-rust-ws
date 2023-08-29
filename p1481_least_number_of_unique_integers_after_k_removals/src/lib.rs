use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

struct Solution;
impl Solution {
    pub fn find_least_num_of_unique_ints_a(arr: Vec<i32>, k: i32) -> i32 {
        let mut map = HashMap::new();
        for num in arr {
            *map.entry(num).or_insert(0) += 1;
        }
        let mut heap = BinaryHeap::new();
        for (num, count) in map {
            heap.push(Reverse((count, num)));
        }
        for _ in 0..k {
            let (count, num) = heap.pop().unwrap().0;
            if (count - 1) > 0 {
                heap.push(Reverse((count - 1, num)));
            }
        }
        heap.len() as i32
    }

    pub fn find_least_num_of_unique_ints_b(arr: Vec<i32>, mut k: i32) -> i32 {
        let mut map = HashMap::new();

        for &n in arr.iter() {
            *map.entry(n).or_insert(0) += 1;
        }

        let mut counts: Vec<i32> = map.into_iter().map(|(_, v)| v).collect();
        counts.sort_by_key(|n| Reverse(*n));

        while k > 0 {
            let n = counts.len();
            if counts[n - 1] <= k {
                let c = counts.pop().unwrap();
                k -= c;
            } else {
                break;
            }
        }

        counts.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let arr = vec![5, 5, 4];
        let result1 = Solution::find_least_num_of_unique_ints_a(arr.clone(), 1);
        let result2 = Solution::find_least_num_of_unique_ints_b(arr.clone(), 1);
        assert_eq!(result1, 1);
        assert_eq!(result2, 1);
    }

    #[test]
    fn case_02() {
        let arr = vec![4, 3, 1, 1, 3, 3, 2];
        let result1 = Solution::find_least_num_of_unique_ints_a(arr.clone(), 3);
        let result2 = Solution::find_least_num_of_unique_ints_b(arr.clone(), 3);
        assert_eq!(result1, 2);
        assert_eq!(result2, 2);
    }
}
