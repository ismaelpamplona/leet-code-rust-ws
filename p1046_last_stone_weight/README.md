# p1046_last_stone_weight
[https://leetcode.com/problems/last-stone-weight/](https://leetcode.com/problems/last-stone-weight/)

## Initial provided code
```Rust
impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        
    }
}
```

## First approach - Heap

`n``: number of elements
- 
### time complexity: $O(n log n)$
- Building the binary heap from the input vector of stones takes $O(n)$ time, where `n` is the number of stones.
- Inside the while loop, the most time-consuming operation is popping two elements from the heap. Each pop operation takes $O(log n)$ time in a binary heap.
- The worst-case scenario is that you perform $n-1$ pops from the heap.
Therefore, the overall time complexity of this code is $O(n)$ (for building the heap) + $O((n-1) * log n)$ (for the while loop) = $O(n log n)$.

### time complexity: $O(n)$
- The space complexity is determined by the binary heap created from the input vector. In this case, the binary heap stores a copy of the input vector, so the space complexity is $O(n)$ for the heap.
- The rest of the variables used in the function have constant space requirements, so they don't significantly impact the overall space complexity.





