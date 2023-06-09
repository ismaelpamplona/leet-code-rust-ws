# p3_longest_substring_without_repeating_characters
[https://leetcode.com/problems/longest-substring-without-repeating-characters/](https://leetcode.com/problems/longest-substring-without-repeating-characters/)

Given a string s, find the length of the longest substring without repeating characters.

## Initial provided code
```Rust
impl Solution {
    pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
        
    }
}
```

So, at this point I know:
1. the parameter type; and
2. the return type;

## First approach - Hashing

- n = number of elements
- time complexity: $O(2n) = O(n)$
- space complexity: $O(min(m, n))$