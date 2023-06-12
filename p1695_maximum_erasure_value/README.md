# p1695_maximum_erasure_value
[https://leetcode.com/problems/maximum-erasure-value/](https://leetcode.com/problems/maximum-erasure-value/)

## Initial provided code
```Rust
impl Solution {
    pub fn maximum_unique_subarray(nums: Vec<i32>) -> i32 {
        
    }
}
```

## First approach - Two pointers, prefix sum and HashSet

- n = number of elements
- time complexity: $O(n)$
- space complexity: $O(n)$

## Second approach - Two pointers with boolean array

- n = number of elements
- k = range of non-negative numbers
- time complexity: $O(n)$
- space complexity: $O(k)$