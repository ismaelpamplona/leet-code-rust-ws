# p973_k_closest_points_to_origin
[https://leetcode.com/problems/k-closest-points-to-origin/](https://leetcode.com/problems/k-closest-points-to-origin/)

## Initial provided code
```Rust
impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        
    }
}
```

## First approach - Min Heap

### time complexity: $O(n \cdot \log k)$
- Operations on a `heap`` cost logarithmic time relative to its size. 
- Because our `heap` is limited to a size of `k`, operations cost at most $O(\log k)$. 
- We iterate over `points`, performing one or two heap operations at each iteration.
- We iterate `n` times, performing up to $\log k$ work at each iteration, giving us a time complexity of $O(n \cdot \log k)$.


### space complexity: $O(k)$
- The `heap` uses $O(k)$ space.
- The `result` uses $O(k)$ space.


