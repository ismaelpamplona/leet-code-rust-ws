# p78_subsets
[https://leetcode.com/problems/subsets/](https://leetcode.com/problems/subsets/)

## Initial provided code
```Rust
impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        
    }
}
```
## First approach - Backtracking

- n = number of numbers

### time complexity: aprox. $O(n \cdot n!)$

- The initial call to backtrack (the "root" of the tree), makes `n` calls.
- Each of those calls makes `n - 1` calls (avoiding duplicates), and each of those make `n - 2`, and so on. The exact time complexity of the algorithm is actually quite complicated, but we can use the logic just stated to estimate it to be around $O(n \cdot n!)$.

### space complexity: $O(n)$
- There is an extra factor of n because we need to copy each permutation. The space complexity is $O(n)$ for `cur` and the recursion call stack.



