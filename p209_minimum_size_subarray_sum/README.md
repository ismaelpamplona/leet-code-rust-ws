# p209_minimum_size_subarray_sum
[https://leetcode.com/problems/minimum-size-subarray-sum/](https://leetcode.com/problems/minimum-size-subarray-sum/)

## Initial provided code
```Rust
impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        
    }
}
```

So, at this point I know:
1. the parameter type; and
2. the return type;

## First approach

1. Start the minimum length variable (`min_length`) with a impossible value `nums.len() + 1)`;
2. If the `min_length` stays equals to `nums.len() + 1)`, then return 0. In this case we didn't find any subarray whose sum is greater than or equal to target;
3. Iterate in `nums` vector to find the subarrays sums;
4. While `sum >= target`, the length (sliding window) is `right - left + 1`. Get the minimum value between `min_length` and `right - left + 1`. In the very first while interation, the `min_length` variable has that impossible value (`nums.len() + 1)`) that we setted on the first step, so the result of `right - left + 1` is certainly less than `nums.len() + 1)`;
5. Subtract the left value from the sum and go to the next left value `left += 1`

```Rust
pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
  let mut min_length = (nums.len() + 1) as i32;
  let mut sum = 0;
  let mut left = 0;

  for right in 0..nums.len() {
      sum += nums[right];

      while sum >= target {
          min_length = min_length.min((right - left + 1) as i32);
          sum -= nums[left];
          left += 1;
      }
  }

  if min_length == (nums.len() + 1) as i32 {
      return 0;
  }

  min_length
}
```

- n = number of elements
- time complexity: O(n)
- space complexity: O(1)