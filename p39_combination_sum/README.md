# p39_combination_sum

[https://leetcode.com/problems/combination-sum/](https://leetcode.com/problems/combination-sum/)

## Initial provided code

```Rust
impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {

    }
}
```

## First approach - Backtracking

### Time Complexity: $O(n^{\frac{t}{m}+1})$

In a backtracking algorithm, we explore all possibilities as if traversing a tree (DFS). The time complexity relates to the total nodes visited.

- Every node processes in constant time, but copying the final combination at leaf nodes takes linear time.
- The breadth of the tree is limited by $n$, the number of candidates.
- The maximum depth is $\frac{t}{m}$, where we repeatedly add the smallest element.
- The theoretical maximum nodes in such a tree is $n^{\frac{t}{m}+1}$, but in practice, it's less since not all paths are explored.

### Space Complexity: $O(\frac{t}{m})$

The space complexity is driven by:

- The recursion depth, which at most is $\frac{t}{m}$, indicating the stack size.
- The space to hold one combination of numbers, also $O(\frac{t}{m})$.

Thus, the total space needed is $O(\frac{t}{m})$, excluding the space to store all the final results.
