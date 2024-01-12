# p128_longest_consecutive_sequence

[https://leetcode.com/problems/longest-consecutive-sequence/](https://leetcode.com/problems/longest-consecutive-sequence/)

## Initial provided code

```Rust
impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {

    }
}
```

## First approach -

### Time Complexity: $O(n)$

Despite the nested loops, the algorithm runs in linear time. The inner `while` loop is only triggered for the start of a new sequence, limiting its total iterations to $n$. Thus, the complexity is $O(n + n) = O(n)$.

### Space Complexity: $O(n)$

Linear space is used for a hash table to facilitate $O(1)$ lookups, making the space complexity $O(n)$.
