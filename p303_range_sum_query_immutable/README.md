# p303_range_sum_query_immutable
[https://leetcode.com/problems/range-sum-query-immutable/](https://leetcode.com/problems/range-sum-query-immutable/)

## Initial provided code
```Rust
struct NumArray {

}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {

    fn new(nums: Vec<i32>) -> Self {
        
    }
    
    fn sum_range(&self, left: i32, right: i32) -> i32 {
        
    }
}

/**
 * Your NumArray object will be instantiated and called as such:
 * let obj = NumArray::new(nums);
 * let ret_1: i32 = obj.sum_range(left, right);
 */
```

So, at this point I know:
1. the parameter type; and
2. the return type;

## First approach - prefix sum

```Rust
fn new(nums: Vec<i32>) -> Self {
  let mut prefix_sum = vec![nums[0]];
  for i in 1..nums.len() {
      prefix_sum.push(nums[i] + prefix_sum[i - 1]);
  }
  Self { nums, prefix_sum }
}
```

- n = number of elements
- time complexity: O(n)
- space complexity: O(n)

```Rust
    fn sum_range(&self, left: i32, right: i32) -> i32 {
        if left == 0 {
            return self.prefix_sum[right as usize];
        } else {
            return self.prefix_sum[right as usize] - self.prefix_sum[left as usize - 1];
        }
    }
```

- n = number of elements
- time complexity: O(n)
- space complexity: O(1)