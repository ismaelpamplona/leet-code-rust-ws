# p496_next_greater_element_i

[https://leetcode.com/problems/next-greater-element-i/](https://leetcode.com/problems/next-greater-element-i/)

## Initial provided code
```Rust
impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        
    }
}
```

## First approach - Monotonic Stack + sliding window
- n: number of `nums2` elements
- m: number of `nums1` elements, and `nums1` is a subset of `nums2` ($m ≤ n$)
- time complexity: $O(n + m) = O(n)$
- space complexity: $O(2n) = O(n)$

