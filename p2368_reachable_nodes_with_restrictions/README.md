# p2368_reachable_nodes_with_restrictions
[https://leetcode.com/problems/reachable-nodes-with-restrictions/](https://leetcode.com/problems/reachable-nodes-with-restrictions/)

## Initial provided code
```Rust
impl Solution {
    pub fn reachable_nodes(n: i32, edges: Vec<Vec<i32>>, restricted: Vec<i32>) -> i32 {
        
    }
}
```
## First approach - DFS - iterative

- n = number of rows
- time complexity: $O(n)$
  - In typical DFS search, the time complexity is $O(v + e)$, where `(v,e)` are the number of vertices and edges. In this problem, there are `n` nodes and `n − 1` edges.
  - The overall time complexity is $O(n)$.
- space complexity: $O(n)$
  - `graph` hash map to stores `n - 1` edges: $O(n)$ space.
  - `seen` hash set, keeps track of the visited nodes: $O(n)$ space.
  - `stack` stores all the nodes to be visited: $O(n)$ space.
  - Therefore, the overall space complexity is $O(n)$.

## Second approach - DFS - recursive

- n = number of rows
- time complexity: $O(n)$
  - In typical DFS search, the time complexity is $O(v + e)$, where `(v,e)` are the number of vertices and edges. In this problem, there are $n$ nodes and $n−1$ edges.
  - The overall time complexity is $O(n)$.
- space complexity: $O(n)$
  - `graph` hash map to stores `n - 1` edges: $O(n)$ space.
  - `seen` hash set, keeps track of the visited nodes: $O(n)$ space.
  - The recursive function: $O(n)$ space.
  - Therefore, the overall space complexity is $O(n)$.







