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

## First approach - sliding 

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


## Second approach - prefix + hashing 

1. Track the number of odd numbers we have seen in the current prefix. 
2. If `curr - k` exists, it means that there was a prefix earlier with `curr - k` odd numbers. The current prefix has curr odd numbers.
3. The difference between them represents the number of odd numbers in the subarray between the prefixes, which is `curr - (curr - k) = k` odd numbers.


```Rust
pub fn number_of_subarrays(nums: Vec<i32>, k: i32) -> i32 {
   let mut map: HashMap<i32, i32> = HashMap::new();
   map.insert(0, 1);
   let mut ans = 0;
   let mut cur_prefix_count = 0;
   for i in 0..nums.len() {
      cur_prefix_count += nums[i] % 2;
      if let Some(value) = map.get(&(cur_prefix_count - k)) {
          ans += value;
      }
      let entry = map.entry(cur_prefix_count).or_insert(0);
      *entry += 1;
   }
   ans
}
```

- n = number of elements
- time complexity: O(n)
- space complexity: O(n)