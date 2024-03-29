# p523_continuous_subarray_sum
[https://leetcode.com/problems/continuous-subarray-sum/](https://leetcode.com/problems/continuous-subarray-sum/)

Given an integer array nums and an integer k, return true if nums has a good subarray or false otherwise.

A good subarray is a subarray where:

- its length is at least two, and
- the sum of the elements of the subarray is a multiple of k.

Note that:

- A subarray is a contiguous part of the array.
- An integer x is a multiple of k if there exists an integer n such that $x = n * k$. 0 is always a multiple of k.

## Initial provided code
```Rust
impl Solution {
    pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
        
    }
}
```

So, at this point I know:
1. the parameter type; and
2. the return type;

## First approach - Hashing

- n = number of elements in `nums`
- time complexity: $O(n)$
- space complexity: O(min{n, k})

The size of a hash map does not exceed $n+1$. It also does not exceed $k$ because there are only $k$ possible remainders.