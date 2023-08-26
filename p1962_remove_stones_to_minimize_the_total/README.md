# p1962_remove_stones_to_minimize_the_total
[https://leetcode.com/problems/remove-stones-to-minimize-the-total/](https://leetcode.com/problems/remove-stones-to-minimize-the-total/)

## Initial provided code
```Rust
impl Solution {
    pub fn min_stone_sum(piles: Vec<i32>, k: i32) -> i32 {
        
    }
}
```

## First approach - Heap

`n`: number of elements
`k`: number of operations
- 
### time complexity: $O(n + k * log n)$
- Vector `piles`` can be converted to a heap in linear time - $O(n)$.
- After converting the input to a heap, we perform $k$ heap operations. Each heap operation costs $(log⁡ n)$, which gives us a time complexity of $O(k * log⁡ n)$. So, $O(n + k log n)$.

### space complexity: $O(n)$
- The heap's length is equal to $n$, which is all the extra space we use.





