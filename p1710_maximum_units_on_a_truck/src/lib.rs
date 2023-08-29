use std::collections::BinaryHeap;

struct Solution;
impl Solution {
    pub fn maximum_units(box_types: Vec<Vec<i32>>, mut truck_size: i32) -> i32 {
        let mut heap: BinaryHeap<(i32, i32)> =
            box_types.into_iter().map(|v| (v[1], v[0])).collect();
        let mut unit_count = 0;
        while let Some(box_type) = heap.pop() {
            let box_count = truck_size.min(box_type.1);
            unit_count += box_count * box_type.0;
            truck_size -= box_count;
            if truck_size == 0 {
                break;
            }
        }
        unit_count
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let box_types = vec![vec![1, 3], vec![2, 2], vec![3, 1]];
        let truck_size = 4;
        let result1 = Solution::maximum_units(box_types, truck_size);
        assert_eq!(result1, 8);
    }

    #[test]
    fn case_02() {
        let box_types = vec![vec![5, 10], vec![2, 5], vec![4, 7], vec![3, 9]];
        let truck_size = 10;
        let result1 = Solution::maximum_units(box_types, truck_size);
        assert_eq!(result1, 91);
    }
}
