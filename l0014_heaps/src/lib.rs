use std::collections::BinaryHeap;
struct Solution;
impl Solution {
    fn heap_interface() {
        // Declaration: Create a min heap using BinaryHeap
        let mut heap = BinaryHeap::new();

        // Add to heap
        heap.push(1);
        heap.push(2);
        heap.push(3);

        // Check maximum element
        if let Some(max) = heap.peek() {
            println!("Maximum element: {}", max); // 3
        }

        // Pop maximum element
        if let Some(max) = heap.pop() {
            println!("Popped maximum element: {}", max); // 3
        }

        // Get size
        let size = heap.len();
        println!("Heap size: {}", size); // 2

        // Bonus: convert a vector to a heap in linear time
        let mut nums = vec![43, 2, 13, 634, 120];
        let mut heap_from_vec = BinaryHeap::from(nums);

        // Now, you can use push and pop on heap_from_vec
        if let Some(max) = heap_from_vec.pop() {
            println!("Popped maximum element from heap_from_vec: {}", max); // 634
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_heap_interface() {
        Solution::heap_interface();
    }
}
