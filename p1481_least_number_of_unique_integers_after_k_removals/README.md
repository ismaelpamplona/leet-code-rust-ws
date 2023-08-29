# p1481_least_number_of_unique_integers_after_k_removals
[https://leetcode.com/problems/least-number-of-unique-integers-after-k-removals/](https://leetcode.com/problems/least-number-of-unique-integers-after-k-removals/)

## Initial provided code
```Rust
impl Solution {
    pub fn find_least_num_of_unique_ints(arr: Vec<i32>, k: i32) -> i32 {
        
    }
}
```

## First approach - Greedy + hashmap + min heap

`n`: number of elements
 
### time complexity: $O(n \cdot \log n)$
- Each iteration in our while loop runs in $O(1)$, and it can also run at most $n$ times, giving a final time complexity of $O(n \cdot \log n)$. 

### space complexity: $O(n)$
- The space complexity is $O(n)$ due to the hash map.





