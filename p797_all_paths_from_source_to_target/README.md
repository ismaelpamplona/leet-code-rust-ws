# p797_all_paths_from_source_to_target
[https://leetcode.com/problems/all-paths-from-source-to-target/](https://leetcode.com/problems/all-paths-from-source-to-target/)

## Initial provided code
```Rust
impl Solution {
    pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        
    }
}
```

## First approach - Backtracking


### Time Complexity: $O(2^n \cdot n)$

- As we calculated shortly before, there could be at most $2^{n-1} - 1$ possible paths in the graph.
- For each path, there could be at most $n-2$ intermediate nodes, i.e., it takes $O(n)$ time to build a path.
- To sum up, a loose upper-bound on the time complexity of the algorithm would be $(2^{n-1} - 1) \cdot O(n) = O(2^n \cdot n)$, where we consider it takes $O(n)$ efforts to build each path.
- To estimate a lower-bound on the time complexity, you can imagine an extreme but valid input: the edge set of the graph is $\{\langle i,j\rangle| 0 \le i < j < n\}$, that is, there exists an edge from node $i$ to $j$ if and only if $i < j$. In this case, we can arbitrarily build a set of nodes from 1 to $n-1$ and construct a valid path by adding the starting point 0 and the end point $n-1$.
- For each path of $k$ intermediate nodes, we have to take $O(k)$ time to build and deep copy the path to the result set. Thus, the total time complexity is at least $\sum_{k=0}^{n-2}{k \cdot {n-2 \choose k}} = 2^{n-3} \cdot (n-2)$, which is still $O(2^n \cdot n)$.







