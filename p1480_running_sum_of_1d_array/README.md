# p1480_running_sum_of_1d_array
[https://leetcode.com/problems/running-sum-of-1d-array/](https://leetcode.com/problems/running-sum-of-1d-array/)

## Initial provided code
```Rust
impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        
    }
}
```

So, at this point I know:
1. the parameter type;
2. the return type; and 
3. that LeetCode tests uses a vector as input.

## First approach - Iterative

```Rust
 pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
     let mut running_sum: Vec<i32> = vec![];
     let mut curr_sum = 0;
     for n in nums {
         curr_sum = n + curr_sum;
         running_sum.push(curr_sum);
     }
     running_sum
 }
```
Since we don't use any additional space to find the running sum. Note that we do not take into consideration the space occupied by the output array.

- n = number of elements
- time complexity: O(n)
- space complexity: O(1)