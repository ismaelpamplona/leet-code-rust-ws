# p283_move_zeroes
[https://leetcode.com/problems/move-zeroes](https://leetcode.com/problems/move-zeroes)

## Initial provided code

```Rust
impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {

    }
}
```

## First approach - count zeros (operation sub-optimal)


```Rust
pub fn move_zeroes(nums: &mut Vec<i32>) {
    let mut zero_count = 0;
    let mut i: i32 = 0;

    while i < nums.len() as i32 {
        if nums[i as usize] == 0 {
            zero_count += 1;
            nums.remove(i as usize);
            i -= 1;
        }
        i += 1;
    }

    for _ in 0..zero_count {
        nums.push(0);
    }
}
```

- n = number of elements
- time complexity: O(n)
- space complexity: O(1)

## Second approach (operation and space optimal)

```Rust
pub fn move_zeroes(nums: &mut Vec<i32>) {
    let mut last_non_zero_found_at = 0;

    for cur in 0..nums.len() {
        if nums[cur] != 0 {
            nums.swap(last_non_zero_found_at, cur);
            last_non_zero_found_at += 1;
        }
    }
}
```

- n = number of elements
- time complexity: O(n)
- space complexity: O(1)