# p215_kth_largest_element_in_an_array
[https://leetcode.com/problems/kth-largest-element-in-an-array/](https://leetcode.com/problems/kth-largest-element-in-an-array/)

## Initial provided code
```Rust
impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        
    }
}
```

## First approach - Min Heap

### time complexity: $O(n \cdot \log k)$
- Operations on a `heap`` cost logarithmic time relative to its size. 
- Because our `heap` is limited to a size of `k`, operations cost at most $O(\log k)$. 
- We iterate over `nums``, performing one or two heap operations at each iteration.
- We iterate `n` times, performing up to $\log k$ work at each iteration, giving us a time complexity of $O(n \cdot \log k)$.


### space complexity: $O(k)$
- The `heap` uses $O(k)$ space.


