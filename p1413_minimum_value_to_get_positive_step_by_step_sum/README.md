# p1413_minimum_value_to_get_positive_step_by_step_sum
[https://leetcode.com/problems/minimum-value-to-get-positive-step-by-step-sum/description/](https://leetcode.com/problems/minimum-value-to-get-positive-step-by-step-sum/description/)

## Initial provided code

```Rust
impl Solution {
    pub fn min_start_value(nums: Vec<i32>) -> i32 {

    }
}
```

So, at this point I know:
1. the parameter type; and
2. the return type;

## Examples
1. Input: nums = [-3,2,-3,4,2] and Output: 5

Taking the vector `nums = [-3, 2, -3, 4, 2]` as an example, let's calculate the prefix sum:

```Rust
let nums = [-3, 2, -3, 4, 2]
let prefix_sum = [-3, -1, -4, 0, 2]
let prefix_and_start_number = [5 + (-3), 5 + (-1), 5 + (-4), 5 + 0, 5 + 2]
```

So the solution resumes in get the lower number of the prefix sum and find the minimum number that respect the condition: `n + start_point >= 1`. In this case, the lower number is `-4` and the minimum start point is `1 - (-4) = 5`.


## First approach - prefix sum

```Rust
pub fn min_start_value(nums: Vec<i32>) -> i32 {
  let mut min_value = 0;
  let mut sum = 0;
  for i in 0..nums.len() {
      sum += nums[i];
      min_value = cmp::min(min_value, sum);
  }
  -min_value + 1
}
```

- n = number of elements
- time complexity: O(n)
- space complexity: O(1)