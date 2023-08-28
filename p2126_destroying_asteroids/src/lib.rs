use std::{cmp::Reverse, collections::BinaryHeap};

#[allow(dead_code)]
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn asteroids_destroyed_heap(mass: i32, asteroids: Vec<i32>) -> bool {
        let mut heap = BinaryHeap::from(asteroids.iter().map(|&x| Reverse(x)).collect::<Vec<_>>());
        let mut cur_mass = mass as i64;
        while let Some(Reverse(a)) = heap.pop() {
            if a as i64 > cur_mass {
                return false;
            }
            cur_mass += a as i64;
        }
        true
    }

    pub fn asteroids_destroyed_sort(mass: i32, asteroids: Vec<i32>) -> bool {
        let mut astes = asteroids.clone();
        astes.sort();
        let mut cur_mass = mass as i64;
        for a in astes {
            if a as i64 > cur_mass {
                return false;
            }
            cur_mass += a as i64;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let asteroids = vec![3, 9, 19, 5, 21];
        let result1 = Solution::asteroids_destroyed_heap(10, asteroids.clone());
        let result2 = Solution::asteroids_destroyed_sort(10, asteroids.clone());
        assert_eq!(result1, true);
        assert_eq!(result2, true);
    }

    #[test]
    fn case_02() {
        let asteroids = vec![4, 9, 23, 4];
        let result1 = Solution::asteroids_destroyed_heap(5, asteroids.clone());
        let result2 = Solution::asteroids_destroyed_sort(5, asteroids.clone());
        assert_eq!(result1, false);
        assert_eq!(result2, false);
    }
}
