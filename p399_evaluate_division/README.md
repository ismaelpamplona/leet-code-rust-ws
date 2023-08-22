# p399_evaluate_division
[https://leetcode.com/problems/evaluate-division/](https://leetcode.com/problems/evaluate-division/)

## Initial provided code
```Rust
impl Solution {
    pub fn calc_equation(equations: Vec<Vec<String>>, values: Vec<f64>, queries: Vec<Vec<String>>) -> Vec<f64> {
        
    }
}
```

## First approach - BFS

If we have n as the number of variables given from equations (which is linear with equations.length in the worst case) and q as the length of queries, then we have a time complexity of $O(q ⋅ (n + e))$. 

Each call to answerQuery is a traversal on the graph we built, which as we know costs the number of nodes plus the number of edges n + e. We perform q traversals. If we aren't counting the space used for the output as extra space, then the space complexity is $O(n + e)$ for building `graph`, `seen`, and the recursion call `stack`.

- $n$: number of variables
- $q$: `queries` length
- $e$: `equations` length
- time complexity: $O(q ⋅ (n + e))$
- space complexity: $O(n + e)$


