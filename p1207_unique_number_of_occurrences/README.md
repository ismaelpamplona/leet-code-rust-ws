# p1207_unique_number_of_occurrences
[https://leetcode.com/problems/unique-number-of-occurrences/](https://leetcode.com/problems/unique-number-of-occurrences/)

## Initial provided code
```Rust
impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        
    }
}
```

So, at this point I know:
1. the parameter type;
2. the return type; and 
3. that LeetCode tests uses a vector as input.

## First approach - HashMap + HashSet

- n = number of nodes
- time complexity: $O(2n) = O(n)$
- space complexity: $O(2n) = O(n)$
