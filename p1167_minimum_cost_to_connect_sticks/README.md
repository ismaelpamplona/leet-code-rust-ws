# p1167_minimum_cost_to_connect_sticks
[https://leetcode.com/problems/minimum-cost-to-connect-sticks/](https://leetcode.com/problems/minimum-cost-to-connect-sticks/)

## Initial provided code
```Rust
impl Solution {
    pub fn connect_sticks(sticks: Vec<i32>) -> i32 {
        
    }
}
```

## First approach - Heap

`n`: number of elements
 
### time complexity: $O(n \log n)$
1. Adding `n` elements to the priority queue will be $O(n \log n)$.
2. We `pop` two of the smallest elements and then `push` one element to the heap until we are left with one element. Since each such operation will reduce one element from the heap, we will perform $n - 1$ such operations. Now, we know that both `push` and `pop` operations take $O(\log n)$ in heap, therefore, complexity of this step will be $O(n \log n)$.

### space complexity: $O(n)$
- Since we will store $n$ elements in our priority queue.





