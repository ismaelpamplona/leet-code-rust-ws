# p713_subarray_product_less_than_k
[https://leetcode.com/problems/subarray-product-less-than-k/](https://leetcode.com/problems/subarray-product-less-than-k/)

## Initial provided code
```Rust
impl Solution {
    pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {

    }
}
```

So, at this point I know:
1. the parameter type; and
2. the return type;

## First approach - Sliding Window

Using the first test parameters `[10, 5, 2, 6]` as a starting point, I realize that for each index, the number of subarrays ending at that index is the length of the sliding window after reaching that index. 

0. `[10]` (10)
1. `[5]` (5), `[10, 5]` (50)
2. `[2]` (2), `[5, 2]` (10)
3. `[6]` (6), `[2, 6]` (12), `[5, 2, 6]` (60)


### Solution

```Rust
 pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
     if k <= 1 {
         return 0;
     }

     let (mut ans, mut curr_product, mut left) = (0, 1, 0);

     for right in 0..nums.len() {
         curr_product *= nums[right];

         while curr_product >= k {
             curr_product /= nums[left];
             left += 1;
         }

         ans += right - left + 1;
     }
     ans as i32
 }
```

- n = number of elements
- time complexity: O(n)
- space complexity: O(1)