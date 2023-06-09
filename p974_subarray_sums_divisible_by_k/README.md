# p974_subarray_sums_divisible_by_k
[https://leetcode.com/problems/subarray-sums-divisible-by-k/](https://leetcode.com/problems/subarray-sums-divisible-by-k/)

Given an integer array nums and an integer k, return the number of non-empty subarrays that have a sum divisible by k.

A subarray is a contiguous part of an array.

## Initial provided code
```Rust
impl Solution {
    pub fn subarrays_div_by_k(nums: Vec<i32>, k: i32) -> i32 {
        
    }
}
```

So, at this point I know:
1. the parameter type; and
2. the return type;

## First approach - Hashing

- n = number of elements
- time complexity: $O(n)$
- space complexity: $O(min(n, k))$