# p791_custom_sort_string
[https://leetcode.com/problems/custom-sort-string/](https://leetcode.com/problems/custom-sort-string/)

## Initial provided code
```Rust
impl Solution {
    pub fn custom_sort_string(order: String, s: String) -> String {
        
    }
}
```

## First approach - map + set
- $n$: length of `order`
- $m$: length of `s`
- $w$: number of unique words in `s`
- time complexity: $O(n + m)$
- space complexity: $O(m)$
