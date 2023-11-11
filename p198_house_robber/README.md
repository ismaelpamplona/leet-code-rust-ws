# p198_house_robber

[https://leetcode.com/problems/house-robber/](https://leetcode.com/problems/house-robber/)

## Initial provided code

```Rust
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {

    }
}
```

## First approach - Dynamic programming

### Time Complexity: $O(n)$

The time complexity is $O(n)$ since we have a loop from $n-2$ down to $0$, and we use the precalculated values of our dynamic programming table to calculate the current value in the table, which is a constant-time operation.

### Space Complexity: $O(1)$

The space complexity is $O(1)$ since we are not using a table to store our values. Simply using two variables will suffice for our calculations.
