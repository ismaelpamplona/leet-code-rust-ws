# p875_koko_eating_bananas
[https://leetcode.com/problems/koko-eating-bananas/](https://leetcode.com/problems/koko-eating-bananas/)

## Initial provided code
```Rust
impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        
    }
}
```
  
## First approach - Sort and count

- `n` = `piles.len()`
- `m` = `*piles.iter().max().unwrap()` - maximum number of bananas in a single pile 
- time complexity: $O(n \cdot \log m)$
- space complexity: $O(1)$

