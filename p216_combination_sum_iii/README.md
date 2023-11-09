# p216_combination_sum_iii

[https://leetcode.com/problems/combination-sum-iii/](https://leetcode.com/problems/combination-sum-iii/)

## Initial provided code

```Rust
impl Solution {
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {

    }
}
```

## First approach - Backtracking

# Complexity Analysis

Let's denote `k` as the number of digits in a combination.

## Time Complexity: $O\left(\frac{9! \cdot k}{(9-k)!}\right)$

In a worst-case scenario, we have to explore all potential combinations to the very end, for instance, when the sum `n` is a large number (greater than `81`, which is the sum when all digits from 1 to 9 are used). Initially, we have 9 choices, at the second step we have 8 choices, and so on.

The number of explorations we need to make in the worst case is the permutation of 9 taken `k` at a time, denoted as `P(9, k)`, which equals to: $P(9, k) = \frac{9!}{(9-k)!}$

assuming that `k <= 9`. Note that `k` cannot be greater than 9, as we can't have a combination with non-unique digits.

Each exploration takes constant time, except for the last step where it takes `O(K)` time to make a copy of the combination.

Summing up, the overall time complexity of the algorithm is:

$$ \frac{9!}{(9-k)!} \cdot O(k) = O\left(\frac{9! \cdot k}{(9-k)!}\right) $$

## Space Complexity: $O(k)$

During the backtracking process, we maintain a list to keep the current combination, which holds up to `k` elements. In addition, recursion in backtracking requires additional space for the function call stack, which could stack up to `k` consecutive calls.

The total space complexity, excluding the space for the output list of results, therefore, is: $O(k)$

Note that we did not take into account the space required for storing the final results in the space complexity analysis.
