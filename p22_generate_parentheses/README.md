# p22_generate_parentheses

[https://leetcode.com/problems/generate-parentheses/](https://leetcode.com/problems/generate-parentheses/)

## Initial provided code

```Rust
impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {

    }
}
```

## First approach - Backtracking

### Time Complexity: $O\left(\frac{4^n}{n\sqrt{n}}\right)$

The time complexity of the `generate_parenthesis` function is governed by the number of valid parentheses combinations generated. The function uses a backtracking algorithm to generate all possible combinations of well-formed parentheses strings.

For a pair of parentheses, there are exactly `n` open and `n` close parentheses. The total number of possible combinations of parentheses without the constraint of being well-formed is $2^{2n}$. However, because we are only looking for valid combinations, we don't explore every possible path in the tree of recursive calls.

The number of valid combinations of parentheses is given by the nth Catalan number, which is approximately the order of $O\left(\frac{4^n}{n\sqrt{n}}\right)$. This is because the function only makes a recursive call when it adds a parenthesis that maintains the string as a valid sequence. Therefore, the time complexity is $O\left(\frac{4^n}{n\sqrt{n}}\right)$.

### Space Complexity: $O(n)$

The space complexity is determined by the maximum depth of the recursion tree and the space used to store the `cur` vector.

- **Recursion Depth**: In the worst case, the depth of the recursive call stack will be `2n`, since in the worst case we will place `n` open parentheses and `n` close parentheses to form a valid combination. Therefore, the recursion adds a space complexity of $O(2n)$, which simplifies to $O(n)$.

- **Current Combination Storage**: The `cur` vector, which is used to store the current combination of parentheses, will have at most `2n` characters, since a valid sequence can have no more than `n` open and `n` close parentheses. So, for the `cur` vector, the space complexity is also $O(2n)$, which simplifies to $O(n)$.

Since these two factors are both linear with respect to the input `n`, the overall space complexity of the algorithm is $O(n)$, where `n` is the number of pairs of parentheses.
