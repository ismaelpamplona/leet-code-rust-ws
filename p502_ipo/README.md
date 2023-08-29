# p502_ipo
[https://leetcode.com/problems/ipo/](https://leetcode.com/problems/ipo/)

## Initial provided code
```Rust
impl Solution {
    pub fn find_maximized_capital(k: i32, w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
        
    }
}
```

## First approach - Heap

`n`: number of projects
 
### time complexity: $O((k + N) \cdot \log n)$
- The heap's max size is n, which means its operations are $\log n$ in the worst case, and we do $k + n$ operations (k pop operations, n push operations). 
- The sort at the start also costs $O(n \cdot \log n)$, but this doesn't change the complexity

### space complexity: $O(n)$
- The space complexity is $O(n)$ due to the heap.





