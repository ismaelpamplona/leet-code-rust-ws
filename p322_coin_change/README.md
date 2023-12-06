# p322_coin_change

[https://leetcode.com/problems/coin-change/](https://leetcode.com/problems/coin-change/)

## Initial provided code

```Rust
impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {

    }
}
```

## First approach - Dynamic programming top down

### **Time Complexity:** $O(S \cdot n)$

The algorithm's recursive tree has a height of $S$, where $S$ is the amount, and it solves $S$ subproblems with $n$ iterations each. This results in a time complexity of $O(S \cdot n)$.

### **Space Complexity:** $O(S)$

The memoization table uses extra space proportional to the amount to change, denoted as $S$.
