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
- $n$: slots
- $10$: each slot has 10 options
- $d$: size of `deadends``
- time complexity: $O(10^n * n^2 + d)$
- space complexity: $O(10^n + d)$


