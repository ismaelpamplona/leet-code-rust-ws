# p412_fizz_buzz
[https://leetcode.com/problems/fizz-buzz/](https://leetcode.com/problems/fizz-buzz/)

## Initial provided code
```Rust
impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        
    }
}
```

So, at this point I know:
1. the parameter type; and
2. the return type;

## First approach - Iterative

```Rust
 pub fn fizz_buzz(n: i32) -> Vec<String> {
     let arr: Vec<i32> = (1..=n).collect();
     let mut result: Vec<String> = vec![];
     for v in arr {
         if v % 3 == 0 && v % 5 == 0 {
             result.push(String::from("FizzBuzz"));
         } else if v % 3 == 0 {
             result.push(String::from("Fizz"));
         } else if v % 5 == 0 {
             result.push(String::from("Buzz"));
         } else {
             result.push(v.to_string());
         }
     }
     result
 }
```

## Second approach - Concatenate

```Rust
 pub fn fizz_buzz_conc(n: i32) -> Vec<String> {
     let arr: Vec<i32> = (1..=n).collect();
     let mut result: Vec<String> = vec![];
     for v in arr {
         let mut curr_string = String::from("");
         let div_by_3 = v % 3 == 0;
         let div_by_5 = v % 5 == 0;

         if div_by_3 {
             curr_string = String::from("Fizz");
         }

         if div_by_5 {
             curr_string += "Buzz";
         }

         if curr_string.is_empty() {
             curr_string = v.to_string();
         }
         result.push(curr_string);
     }
     result
 }
 ```


- n = number of elements
- time complexity: O(n)
- space complexity: O(1)


## Third approach - HashMap

```Rust
pub fn fizz_buzz_hash(n: i32) -> Vec<String> {
    let arr: Vec<i32> = (1..=n).collect();
    let mut result: Vec<String> = vec![];
    let mut fizz_buzz_map = HashMap::new();
    fizz_buzz_map.insert(3, "Fizz");
    fizz_buzz_map.insert(5, "Buzz");

    let divisors = vec![3, 5];

    for v in arr {
        let mut curr_string = String::from("");
        for d in &divisors {
            if v % d == 0 {
                curr_string += fizz_buzz_map.get(&d).unwrap_or(&"");
            }
        }

        if curr_string.is_empty() {
            curr_string = v.to_string();
        }

        result.push(curr_string);
    }
    result
}
```

- n = number of elements
- d = number of divisors
- time complexity: O(n*d)
- space complexity: O(1)