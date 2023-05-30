# p1248_count_number_of_nice_subarrays
[https://leetcode.com/problems/count-number-of-nice-subarrays/](https://leetcode.com/problems/count-number-of-nice-subarrays/)

## Initial provided code
```Rust
impl Solution {
    pub fn number_of_subarrays(nums: Vec<i32>, k: i32) -> i32 {
      
    }
}
```

So, at this point I know:
1. the parameter type; and
2. the return type;

## First approach - two pointers

1. Iterate through all the numbers and increment the current odd counter whenever we encounter odd number.
2. Once encountered k odd numbers, increment the left pointer until we reach the first odd number during this process we subtract the odd number and count the number of even numbers.
3. Update the total with the number of even numbers.

```Rust
pub fn number_of_subarrays(nums: Vec<i32>, k: i32) -> i32 {
  let mut left = 0;
  let mut ans = 0;
  let mut sum_odds = 0;
  let mut count = 0;

  for right in 0..nums.len() {
      if nums[right] % 2 != 0 {
          sum_odds += 1;
          count = 0;
      }

      while sum_odds == k {
          if nums[left] % 2 != 0 {
              sum_odds -= 1;
          }
          left += 1;
          count += 1;
      }

      ans += count;
  }

  ans
}
```

- n = number of elements
- time complexity: O(n)
- space complexity: O(1)