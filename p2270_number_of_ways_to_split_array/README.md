# p2270_number_of_ways_to_split_array
[https://leetcode.com/problems/number-of-ways-to-split-array/](https://leetcode.com/problems/number-of-ways-to-split-array/)

## Initial provided code
```Rust
impl Solution {
    pub fn ways_to_split_array(nums: Vec<i32>) -> i32 {
        
    }
}
```

So, at this point I know:
1. the parameter type;
2. the return type; and 
3. that LeetCode tests uses a vector of vectors as input.

## First approach - prefix sum vector

Even the provided code uses a vector of i32 as a parameter, to build the prefix sum vector you need to build a vector of i64 because on test 99, the `nums` vector provided over pass i32 limits.

```Rust
pub fn ways_to_split_array(nums: Vec<i32>) -> i32 {
  let mut prefix: Vec<i64> = vec![nums[0] as i64];
  for i in 1..nums.len() {
      let sum = (nums[i] as i64) + prefix[i - 1];
      prefix.push(sum);
  }

  let mut ans = 0;
  let last = prefix.len() - 1;
  for i in 0..(prefix.len() - 1) {
      if prefix[i] >= (prefix[last] - prefix[i]) {
          ans += 1;
      }
  }
  ans

  // time complexity: O(n)
  // space complexity: O(n)
}
```


- n = nums.len()
- time complexity: O(n)
- space complexity: O(n)

## Second approach - left section calculation on the fly

```Rust
pub fn ways_to_split_array_2(nums: Vec<i32>) -> i32 {
    let mut total: i64 = nums[0] as i64;
    for i in 1..nums.len() {
        total += nums[i] as i64;
    }

    let mut ans = 0;
    let mut left: i64 = 0;
    for i in 0..nums.len() - 1 {
        left += nums[i] as i64;
        let right = total - left;
        if left >= right {
            ans += 1;
        }
    }
    ans

    // time complexity: O(n)
    // space complexity: O(1)
}
```


- n = nums.len()
- time complexity: O(n)
- space complexity: O(1)