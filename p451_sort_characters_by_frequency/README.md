# p451_sort_characters_by_frequency
[https://leetcode.com/problems/sort-characters-by-frequency/](https://leetcode.com/problems/sort-characters-by-frequency/)

## Initial provided code
```Rust
impl Solution {
    pub fn frequency_sort(s: String) -> String {
        
    }
}
```

So, at this point I know:
1. the parameter type;
2. the return type; and 
3. that LeetCode tests uses a vector as input.

## First approach - HashMap + HashSet

- n = number of nodes
- time complexity: $O(n + k \log k)$
- space complexity: $O(n)$
