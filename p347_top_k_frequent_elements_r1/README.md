# p347_top_k_frequent_elements_r1

[https://leetcode.com/problems/top-k-frequent-elements/](https://leetcode.com/problems/top-k-frequent-elements/)

## Initial provided code

```Rust
impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {

    }
}
```

## First approach - Heap

`n`: number of elements

- time complexity: $O(n \cdot \log k)$ if $k < n$ and $O(n)$ in the particular case of $n = k$. That ensures time complexity to be better $O(n \cdot \log n)$
- space complexity: $O(n + k)$, to store the hash map with not more
  $n$ elements and a heap with $k$ elements.
