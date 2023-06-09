# p1748_sum_of_unique_elements
[https://leetcode.com/problems/sum-of-unique-elements/](https://leetcode.com/problems/sum-of-unique-elements/)

## Initial provided code
```Rust
impl Solution {
    pub fn sum_of_unique(nums: Vec<i32>) -> i32 {
        
    }
}
```

So, at this point I know:
1. the parameter type;
2. the return type; and 
3. that LeetCode tests uses a vector as input.

## First approach - HashSet

- n = number of nodes
- time complexity: $O(2n) = O(n)$
- space complexity: $O(n)$
