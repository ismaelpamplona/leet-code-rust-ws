# Binary Search

- Binary search is a search algorithm that runs in $O(\log n)$ in the worst case, where `n` is the size of the search space.
- For binary search to work, your search space usually needs to be **sorted**.
- Binary search trees are based on binary search.
- If you have a sorted `vector` and an element `x`, then in $O(\log n)$ time and $O(1)$ space, binary search can:
   - Find the index of `x` if it is in `vector`
   - Find the first or the last index in which `x` can be inserted to maintain being sorted otherwise

## Implementation

### Logic
1. Declare `let left = 0 as usize;` and `let right = vec.length - 1;`. These variables represent the inclusive bounds of the current search space at any given time. Initially, we consider the entire vector.
1. `while left <= right`:
   - Calculate the middle of the current search space, `let mid = (left + right) / 2;` (floor division)
   - Check `vec[mid]`. There are 3 possibilities:
        - `if vec[mid] == x`, then the element has been found, return.
        - `if vec[mid] > x`, then halve the search space by doing `right = mid - 1`.
        - `if vec[mid] < x`, then halve the search space by doing `left = mid + 1`.
1. If you get to this point without `vec[mid] == x`, then the search was unsuccessful. The left pointer will be at the index where `x` would need to be inserted to maintain vector being sorted.

> Because the search space is halved at every iteration, binary search's worst case time complexity is $O(\log n)$. This makes it an extremely powerful algorithm as logarithmic time is very fast compared to linear time.

### Templates
#### Rust
```rust
fn binary_search(vec: Vec<i32>, target: i32) -> i32 {
   let (mut left, mut right) = (0 as i32, vec.len() as i32 - 1);
   while left <= right {
      let mid = left + (right - left) / 2;
      if vec[mid as usize] == target {
          println!("do something");
          return mid as i32;
      } else if vec[mid as usize] > target {
          right = mid - 1;
      } else {
          left = mid + 1;
      }
   }
   // target is not in arr, but left is at the insertion point
   println!("not found");
   -1
}
```

```rust
use std::cmp::Ordering::{Equal, Greater, Less};
pub fn search_opt(nums: Vec<i32>, target: i32) -> i32 {
  let (mut left, mut right) = (0, nums.len());
  while left < right {
      let mid = left + (right - left) / 2;
      match nums[mid].cmp(&target) {
          Equal => return mid as i32,
          Less => left = mid + 1,
          Greater => right = mid,
      }
  }
  -1
}
```

#### Python
```py
def binary_search(arr, target):
    left = 0
    right = len(arr) - 1
    while left <= right:
        mid = (left + right) // 2
        if arr[mid] == target:
            # do something
            return
        if arr[mid] > target:
            right = mid - 1
        else:
            left = mid + 1
    
    # target is not in arr, but left is at the insertion point
    return left
```

#### Typescript
```ts
function binarySearch(arr: number[], target: number): number {
  let left = 0;
  let right = arr.length - 1;
  while (left <= right) {
    let mid = Math.floor((left + right) / 2);
    if (arr[mid] === target) {
      console.log("do something");
      return mid;
    }
    if (arr[mid] > target) {
      right = mid - 1;
    } else {
      left = mid + 1;
    }
  }
  console.log("not found");
  return -1; // target is not in arr, but left is at the insertion point
}
```

>  In the **Rust, Java and C++** implementations, instead of doing `(left + right) / 2`, we do `left + (right - left) / 2` to **avoid overflow**. The equations are equivalent, but the second one makes sure that no value greater than right is ever stored. In Python and Typescript, numbers don't overflow (or at least, the limit is ridiculously huge), so we are fine with having `left + right` potentially being large.

### Duplicate elements

- If your input has duplicates, you can modify the binary search template to find either the first or the last position of a given element. 
- If target appears multiple times, then:
  
#### Find the left-most index:
```rust
fn binary_search_duplicates_left(vec: Vec<i32>, target: i32) -> i32 {
    let mut left = 0 as usize;
    let mut right = vec.len();
    while left < right {
        let mid = left + (right - left) / 2;
        if vec[mid] >= target {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    left as i32
}
```

#### Find the right-most insertion point (the index of the right-most element plus one):
```rust
fn binary_search_duplicates_right(vec: Vec<i32>, target: i32) -> i32 {
    let mut left = 0 as usize;
    let mut right = vec.len();
    while left < right {
        let mid = left + (right - left) / 2;
        if vec[mid] > target {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    left as i32
}
```


