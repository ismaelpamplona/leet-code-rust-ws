# p1143_longest_common_subsequence

[https://leetcode.com/problems/longest-common-subsequence/](https://leetcode.com/problems/longest-common-subsequence/)

## Initial provided code

```Rust
impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {

    }
}
```

## First approach - Multi-dimensional Dynamic Programming (top down)

### Time Complexity: $O(m \times n)$

- Each subproblem is defined by a pair of indices `(i, j)`, where `i` ranges from `0` to `m` (length of `text1`) and `j` ranges from `0` to `n` (length of `text2`). Thus, there are $O(m \times n)$ unique subproblems.
- Due to memoization, each subproblem is solved exactly once in $O(1)$ time.
- The overall time complexity is therefore $O(m \times n)$.

### Space Complexity: $O(m \times n)$

- A two-dimensional vector `memo` of size `(m + 1) x (n + 1)` is used, resulting in a space complexity of $O(m \times n)$.
- In the worst case, the recursion can go `min(m, n)` levels deep, but this is generally overshadowed by the space taken by the memoization table.
- Consequently, the space complexity of the algorithm is $O(m \times n)$.

## Second approach = Multi-dimensional Dynamic Programming (bottom up)

- **Time Complexity:** $O(n \times m)$, as each cell in the `dp` table is filled once.
- **Space Complexity:** $O(n \times m)$, due to the storage used by the `dp` table.
