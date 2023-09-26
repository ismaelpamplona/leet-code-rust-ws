# p77_combinations
[https://leetcode.com/problems/combinations/](https://leetcode.com/problems/combinations/)

## Initial provided code
```Rust
impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        
    }
}
```
## First approach - Backtracking

- n = number of numbers

### time complexity: $O(\frac {k \cdot n!}  {(n - k)! \cdot k!})$

### space complexity: $O(n)$
- There is an extra factor of n because we need to copy each permutation. The space complexity is $O(n)$ for `cur` and the recursion call stack.



