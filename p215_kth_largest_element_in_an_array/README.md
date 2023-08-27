# p215_kth_largest_element_in_an_array
[https://leetcode.com/problems/kth-largest-element-in-an-array/](https://leetcode.com/problems/kth-largest-element-in-an-array/)

## Initial provided code
```Rust
impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        
    }
}
```

## First approach - Heap

### time complexity:
- find_median: $O(1)$
- add_num: $O(log n)$, where `n` is the number of times `add_sum` has been called


### space complexity: $O(n)$


