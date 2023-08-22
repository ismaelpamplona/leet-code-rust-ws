# p433_minimum_genetic_mutation
[https://leetcode.com/problems/minimum-genetic-mutation/](https://leetcode.com/problems/minimum-genetic-mutation/)

## Initial provided code
```Rust
impl Solution {
    pub fn min_mutation(start_gene: String, end_gene: String, bank: Vec<String>) -> i32 {
        
    }
}
```

## First approach - BFS

- $b$: `bank` length

### time complexity: $O(b)$

- As stated before, this is because we are not converting `bank`` to a set due to the low size. If we did convert bank to a set however, it would still cost $O(B)$ to do so. Checking if a neighbor is in the bank costs $O(B)$ with an array.
- Technically, the BFS runs in constant time because the problem limits the length of the gene strings to `8` and the strings can only have `4` characters. However, let's say the gene strings could have length $n$ and could have $m$ kind of characters. In this problem, we have $n = 8$ and $m = 4$. There would be $m^n$ possible nodes, because for each of the nnn characters, there are $m$ options.
- If we are to analyze the complexity like this, let's assume that we are converting bank to a set prior to the BFS. In that case, the time complexity would be $O(nB + m^n \cdot n^2 \cdot m)$. Converting bank costs $O(nB)$, then there are $m^n$ states that we could visit. At each state, we perform a nested for loop which iterates n⋅mn \cdot mn⋅m times, and also perform string operations which cost $O(n)$.

### space complexity: $O(1)$
  
- Same logic as before, because the problem limits the input explicitly, we technically use constant space.
- However, with the same scenario as above, the space complexity would be $O(nB + m^n)$. Converting bank to a set would create a set that takes up $O(nB)$ space, and then the seen set could grow to $m^n$ size if all states are visited.


