# p1302_deepest_leaves_sum
[https://leetcode.com/problems/deepest-leaves-sum/](https://leetcode.com/problems/deepest-leaves-sum/)

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
    pub fn deepest_leaves_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        
    }
}
```
## First approach - Depth-first search (BFS)

- n = number of nodes
- time complexity: $O(n)$
- space complexity: $O(n)$




