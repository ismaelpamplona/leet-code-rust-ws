# p79_word_search

[https://leetcode.com/problems/word-search/](https://leetcode.com/problems/word-search/)

## Initial provided code

```Rust
impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {

    }
}
```

## First approach - Backtracking

### Time Complexity: $O(n \cdot m \cdot 3^L)$

The time complexity of this algorithm is $O(n \cdot m \cdot 3^L)$, where $L$ is the length of the word to be searched. Each node in the search tree (each cell in the grid) can lead to three other nodes, as we do not revisit the cell we just came from. Thus, we have a branching factor of three, leading to $3^L$ possible paths. Since the search can start from any cell in the $n \times m$ grid, we multiply the branching factor by the number of cells, resulting in the time complexity formula.

The worst-case scenario arises when the grid consists of a single repeated character that matches all but the last character of the word. In such a case, the algorithm is forced to explore almost all possible paths, leading to the maximum time complexity.

### Space Complexity: $O(L)$

For space complexity in Rust, we typically use a `HashSet` to keep track of visited cells. This results in a space complexity of $O(L)$, with $L$ being the length of the word. The recursion call stack depth is also bounded by $L$, as we cannot recurse deeper than the length of the word being searched.

An alternative approach in Rust might involve a mutable boolean matrix to track visited cells, which would have a space complexity of $O(n \cdot m)$. This matrix would be separate from the recursion stack and would store the visitation state of each cell independently.
