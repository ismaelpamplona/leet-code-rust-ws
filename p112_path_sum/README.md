# p112_path_sum
[https://leetcode.com/problems/path-sum/](https://leetcode.com/problems/path-sum/)

## Initial provided code
```Rust
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        
    }
}
```
## First approach - Depth-first search (DFS) (recursion with a insider func)

- n = number of nodes
- time complexity: $O(n)$
- space complexity: $O(log(n))$

## Second approach - Depth-first search (DFS) (recursion)

- n = number of nodes
- time complexity: $O(n)$
- space complexity: $O(log(n))$

## Third approach - Depth-first search (DFS) (iteration)

- n = number of nodes
- time complexity: $O(n)$
- space complexity: $O(log(n))$

