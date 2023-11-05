# p52_n_queens_ii

[https://leetcode.com/problems/n-queens-ii/](https://leetcode.com/problems/n-queens-ii/)

## Initial provided code

```Rust
impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {

    }
}
```

## First approach - Backtracking

The algorithm utilizes backtracking to solve the N-Queens problem, placing queens on a chessboard so no two can attack each other.

### Time Complexity: $\tilde{O}(n!)$

The exact time complexity is unknown, but it's approximately estimated to be $O(n!)$. This estimate comes from considering that the first queen has $n$ positions to choose from, the next has at most $n - 2$, and so on, reducing the options by two for each subsequent row because of the constraints imposed by columns and diagonals.

### Space Complexity: $O(n)$

The space complexity is $O(n)$. This is due to the storage requirements of three `HashSet`s that keep track of the columns, diagonals, and anti-diagonals affected by the placement of each queen. As each set can have at most $n$ elements, and the recursion depth also goes up to $n$, the space complexity is linear with respect to the size of the board.
