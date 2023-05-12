# p383_ransom_note
[https://leetcode.com/problems/ransom-note/description/](https://leetcode.com/problems/ransom-note/description/)

## Initial provided code
```Rust
impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        
    }
}
```

So, at this point I know:
1. the parameter type; and
2. the return type;

## First approach -

### Solution

```Rust
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
     let mut rn = ransom_note.clone();
     for m in magazine.chars() {
         for (i, r) in rn.chars().enumerate() {
             if m == r {
                 rn = rn[..i].to_string() + &rn[i + 1..];
                 break;
             }
         }
         if rn.len() == 0 {
             break;
         }
     }
     rn.len() == 0
 }
```

- n = number of characters in ransom note
- m = number of characters in magazine
- time complexity: O(nm)
- space complexity: O(n)

## Second approach - HashMap


```Rust
  pub fn can_construct_hash(ransom_note: String, magazine: String) -> bool {
      let mut magazine_map = HashMap::new();

      for m in magazine.chars() {
          let count = magazine_map.entry(m).or_insert(0);
          *count += 1;
      }

      for r in ransom_note.chars() {
          if let Some(value) = magazine_map.get_mut(&r) {
              if *value > 1 {
                  *value -= 1;
              } else {
                  magazine_map.remove(&r);
              }
          } else {
              return false;
          }
      }

      true
  }
 ```


- m = number of characters in magazine
- k = distinct characters in magazine
- time complexity: O(m)
- space complexity: 
$$
O(k), k <= 26, k -> 1, O(1)
$$
- 

