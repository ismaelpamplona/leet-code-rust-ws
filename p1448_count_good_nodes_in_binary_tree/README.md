# p1448_count_good_nodes_in_binary_tree
[https://leetcode.com/problems/count-good-nodes-in-binary-tree/](https://leetcode.com/problems/count-good-nodes-in-binary-tree/)

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
    pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        
    }
}
```
## First approach - Depth-first search (DFS) (recursion with a insider func)

- n = number of nodes
- time complexity: $O(n)$
- space complexity: $O(n)$


