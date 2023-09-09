# p1231_divide_chocolate
[https://leetcode.com/problems/divide-chocolate/](https://leetcode.com/problems/divide-chocolate/)

- You have one chocolate bar that consists of some chunks. Each chunk has its own sweetness given by the array sweetness.

- You want to share the chocolate with your k friends so you start cutting the chocolate bar into k + 1 pieces using k cuts, each piece consists of some consecutive chunks.

- Being generous, you will eat the piece with the minimum total sweetness and give the other pieces to your friends.

- Find the maximum total sweetness of the piece you can get by cutting the chocolate bar optimally.

## Initial provided code
```Rust
impl Solution {
    pub fn maximize_sweetness(sweetness: Vec<i32>, k: i32) -> i32 {
        
    }
}
```
  
## First approach - Binary Search Using BFS

- `n` = `sweetness.len()`
- `k` = maximum possible answer
  
- time complexity: $O(n \cdot \log k)$
- space complexity: $O(1)$
  
