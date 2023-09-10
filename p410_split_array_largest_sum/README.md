# p410_split_array_largest_sum
[https://leetcode.com/problems/split-array-largest-sum/](https://leetcode.com/problems/split-array-largest-sum/)

- Given an integer array nums and an integer k, split nums into k non-empty subarrays such that the largest sum of any subarray is minimized.

- Return the minimized largest sum of the split.

- A subarray is a contiguous part of the array.

## Initial provided code
```Rust
impl Solution {
    pub fn split_array(nums: Vec<i32>, k: i32) -> i32 {
        
    }
}
```
  
## First approach - Binary Search

- `n` = `nums.len()`
- `s` = `nums.iter().sum()`
  
### time complexity: $O(n \cdot \log s)$
- Binary search: $O(\log s)$
- For each iteration we call `is_valid` which takes $O(n)$

### space complexity: $O(1)$

- No extra space is used.

