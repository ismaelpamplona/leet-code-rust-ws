# p199_binary_tree_right_side_view
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
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        
    }
}
```
## First approach - Depth-first search (DFS)

- n = number of nodes
- h =  tree height
- time complexity: $O(n)$
- space complexity: $O(h)$

## Second approach - Breadth-first search (BFS)

- n = number of nodes
- d = tree diameter
- time complexity: $O(n)$
- space complexity: $O(d)$




