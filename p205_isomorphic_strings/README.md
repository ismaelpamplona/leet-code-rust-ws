# p205_isomorphic_strings
[https://leetcode.com/problems/isomorphic-strings/](https://leetcode.com/problems/isomorphic-strings/)

## Initial provided code
```Rust
impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        
    }
}
```

## First approach - map + set
- $n$: length of `s1`
- time complexity: $O(n)$ - The hashmap contains atmost 26 keys.
- space complexity: $26*2*O(1) = O(1)$ - Hashmap contains at most 26 key-value pairs.
