# p1672_richest_customer_wealth
[https://leetcode.com/problems/richest-customer-wealth/](https://leetcode.com/problems/richest-customer-wealth/)

## Initial provided code
```Rust
impl Solution {
    pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        
    }
}
```

So, at this point I know:
1. the parameter type;
2. the return type; and 
3. that LeetCode tests uses a vector of vectors as input.

## First approach - Iterative

```Rust
 pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
     let mut max = 0;
     for cus in accounts {
         let mut cus_wealth = 0;
         for val in cus {
             cus_wealth += val;
         }
         if cus_wealth > max {
             max = cus_wealth;
         }
     }
     max
 }
```


- i = number of lines
- j = number of columns
- time complexity: O(i*j)
- space complexity: O(1)