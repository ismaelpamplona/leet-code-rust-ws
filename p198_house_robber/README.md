# p198_house_robber

[https://leetcode.com/problems/house-robber/](https://leetcode.com/problems/house-robber/)

## Initial provided code

```Rust
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {

    }
}
```

## First approach - Dynamic programming - bottom up

### Time Complexity: $O(n)$

The time complexity is $O(n)$ since we have a loop from $n-2$ down to $0$, and we use the precalculated values of our dynamic programming table to calculate the current value in the table, which is a constant-time operation.

### Space Complexity: $O(1)$

The space complexity is $O(1)$ since we are not using a table to store our values. Simply using two variables will suffice for our calculations.

## Second approach - Dynamic programming - top down

### Time Complexity: $O(n)$

At each state, applying the recurrence relation is $O(1)$. So, the time complexity of this algorithm is $O(n)$, where $n$ is the length of the input array, because we only visit each state once

### Space Complexity: $O(n)$

Because that's how much space we need to cache the results for each state.
