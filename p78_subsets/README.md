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

### time complexity: $O(n \cdot 2 ^n)$

- There are $2 ^n$ subsets, where $m$ is the length of the input array - for each element, we can either take it or not take it.
- For the time complexity, you can think of the algorithm as a DFS on a tree with $2 ^n$ nodes. 
- At each node, we make a copy of cur, so the time complexity is $O(n \cdot 2 ^n)$

### space complexity: $O(n)$
- The space complexity is $O(n)$ for `cur` and the recursion call stack.



