# p1394_find_lucky_integer_in_an_array
[https://leetcode.com/problems/find-lucky-integer-in-an-array/](https://leetcode.com/problems/find-lucky-integer-in-an-array/)

## Initial provided code
```Rust
impl Solution {
    pub fn find_lucky(arr: Vec<i32>) -> i32 {
        
    }
}
```

So, at this point I know:
1. the parameter type;
2. the return type; and 
3. that LeetCode tests uses a vector as input.

## First approach - HashSet

- n = number of nodes
- time complexity: $O(2n) = O(n)$
- space complexity: $O(n)$
