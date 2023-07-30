# p515_find_largest_value_in_each_tree_row
[https://leetcode.com/problems/find-largest-value-in-each-tree-row/](https://leetcode.com/problems/find-largest-value-in-each-tree-row/)

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
    pub fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        
    }
}
```
## First approach - Depth-first search (BFS)

- n = number of nodes
- time complexity: $O(n)$
- space complexity: $O(n)$




