# p1342_number_of_steps_to_reduce_a_number_to_zero
[https://leetcode.com/problems/number-of-steps-to-reduce-a-number-to-zero/](https://leetcode.com/problems/number-of-steps-to-reduce-a-number-to-zero/)

## Initial provided code
```Rust
impl Solution {
    pub fn number_of_steps(num: i32) -> i32 {
        
    }
}
```

So, at this point I know:
1. the parameter type; and
2. the return type;

## First approach

```Rust
 pub fn number_of_steps(num: i32) -> i32 {
     let mut steps = 0;
     let mut quotient = num;
     while quotient > 0 {
         steps += 1;
         if quotient % 2 == 0 {
             quotient /= 2;
         } else {
             quotient -= 1;
         }
     }
     steps
 }

```

## Second approach - Bitwise

```Rust

```

- n = number of elements
- time complexity: O(logn)
- space complexity: O(1)