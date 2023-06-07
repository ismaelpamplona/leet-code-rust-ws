# p771_jewels_and_stones
[https://leetcode.com/problems/jewels-and-stones/](https://leetcode.com/problems/jewels-and-stones/)

You're given strings jewels representing the types of stones that are jewels, and stones representing the stones you have. Each character in stones is a type of stone you have. You want to know how many of the stones you have are also jewels.

Letters are case sensitive, so "a" is considered a different type of stone from "A".

## Initial provided code
```Rust
impl Solution {
    pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
        
    }
}
```

So, at this point I know:
1. the parameter type; and
2. the return type;

## First approach - Hashing

- n = number of elements
- time complexity: O(n)
- space complexity: O(n)