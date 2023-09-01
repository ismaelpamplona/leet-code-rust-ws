# p1631_path_with_minimum_effort
[https://leetcode.com/problems/path-with-minimum-effort/](https://leetcode.com/problems/path-with-minimum-effort/)

## Initial provided code
```Rust
impl Solution {
    pub fn minimum_effort_path(heights: Vec<Vec<i32>>) -> i32 {
        
    }
}
```
  
## First approach - Binary Search Using BFS

- `m` = `heights.len()`
- `n` = `heights[0].len()`
- `k` = maximum number on `heights` vector
  
### time complexity: $O(m \cdot n \cdot \log k)$
- BFS: $O(m \cdot n)$
- Binary Search: $O(\log k)$

### space complexity: $O(m \cdot n)$

- To perform the DFS, we are using $O(m \cdot n)$ space for the stack and seen.

