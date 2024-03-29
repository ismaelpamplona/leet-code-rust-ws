# p1283_find_the_smallest_divisor_given_a_threshold
[https://leetcode.com/problems/find-the-smallest-divisor-given-a-threshold/](https://leetcode.com/problems/find-the-smallest-divisor-given-a-threshold/)

- Given an array of integers nums and an integer threshold, we will choose a positive integer divisor, divide all the array by it, and sum the division's result. Find the smallest divisor such that the result mentioned above is less than or equal to threshold.

- Each result of the division is rounded to the nearest integer greater than or equal to that element. (For example: 7/3 = 3 and 10/2 = 5).

- The test cases are generated so that there will be an answer.

## Initial provided code
```Rust
impl Solution {
    pub fn smallest_divisor(nums: Vec<i32>, threshold: i32) -> i32 {
        
    }
}
```
  
## First approach - Binary Search

- `n` = `nums.len()`
- `m` = maximum element of the `nums` array
  
- time complexity: $O(n \cdot \log m)$
- space complexity: $O(1)$

