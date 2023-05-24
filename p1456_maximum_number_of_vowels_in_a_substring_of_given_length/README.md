# p1456_maximum_number_of_vowels_in_a_substring_of_given_length
[https://leetcode.com/problems/maximum-number-of-vowels-in-a-substring-of-given-length](https://leetcode.com/problems/maximum-number-of-vowels-in-a-substring-of-given-length)

## Initial provided code
```Rust
impl Solution {
    pub fn max_vowels(s: String, k: i32) -> i32 {

    }
}
```

So, at this point I know:
1. the parameter type; and
2. the return type;

## Auxiliary Functions

To solve this problem, I need to write two auxiliary functions:

### 1. Check if a char is vowel

```Rust
fn is_vowel(c: char) -> bool {
  match c {
      'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => true,
      _ => false,
  }
}
```

## First approach - sliding window

I had some problems with the first algorithm version. On the 100th test, the compiler throws a Time Limit Exceeded error. And it was because I was trying to build my logic without converting the String input into a Vector. So for each iteration of my for loop, I was accessing a char using a rust method that iterates again on the String input `let left_char = s.chars().nth(left).unwrap();`.

### Compile Error - Time Limit Exceeded
```Rust
pub fn max_vowels(s: String, k: i32) -> i32 {
  let mut left = 0;
  let mut num_vowels = 0;
  let mut ans = 0;

  for (right, c) in s.char_indices() {
      if num_vowels == k {
          return num_vowels;
      }
      if Self::is_vowel(c) {
          num_vowels += 1;
      }
      
      if right >= k as usize {
          let left_char = s.chars().nth(left).unwrap(); // Compile Error - Time Limit Exceeded
          if Self::is_vowel(left_char) {
              num_vowels -= 1;
          }
          left += 1;
      }

      ans = std::cmp::max(ans, num_vowels);
  }

  ans
}
```

### Corret - Covert the String input into a Vector
```Rust
pub fn max_vowels(s: String, k: i32) -> i32 {
  let sv: Vec<char> = s.chars().collect();
  let mut left = 0;
  let mut curr = 0;
  let mut max = 0;

  for right in 0..sv.len() {
      if curr == k {
          return curr;
      }
      if Self::is_vowel(sv[right]) {
          curr += 1;
      }

      if right >= k as usize {
          if Self::is_vowel(sv[left]) {
              curr -= 1;
          }
          left += 1;
      }

      max = std::cmp::max(max, curr);
  }

  max
}
```

- n = number of elements
- time complexity: O(n)
- space complexity: O(1)