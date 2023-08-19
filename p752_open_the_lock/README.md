# p752_open_the_lock
[https://leetcode.com/problems/open-the-lock/](https://leetcode.com/problems/open-the-lock/)

## Initial provided code
```Rust
impl Solution {
    pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
        
    }
}
```

## First approach - BFS
- $n$: slots
- $10$: each slot has 10 options
- $d$: size of `deadends``
- time complexity: $O(10^n * n^2 + d)$
- space complexity: $O(10^n + d)$


