# p2101_detonate_the_maximum_bombs
[https://leetcode.com/problems/detonate-the-maximum-bombs/](https://leetcode.com/problems/detonate-the-maximum-bombs/)

## Initial provided code
```Rust
impl Solution {
    pub fn maximum_detonation(bombs: Vec<Vec<i32>>) -> i32 {
        
    }
}
```

## First approach - BFS

- n: number of elements
### time complexity: $O(n³)$
- There are n nodes and at most $n²$ edges in this problem.
  - Building graph takes $O(n²)$ time.
  - Each node is enqueued and dequeued once, it takes $O(n)$ to handle all nodes.
  - For each node, we may need to explore up to $n−1$ edges to find all its neighbors. Since there are $n$ nodes, the total number of edges we explore is at most $n(n−1) = O(n²)$.
- We need to perform $n$ breadth-first searches.

### space complexity: $O(n²)$
- There are at $O(n^2)$ edges stored in graph.
- `q` can store up to $n$ nodes.


