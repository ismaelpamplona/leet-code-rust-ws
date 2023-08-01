# p98_validate_binary_search_tree
[https://leetcode.com/problems/validate-binary-search-tree/](https://leetcode.com/problems/validate-binary-search-tree/)

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
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        
    }
}
```
## First approach - Depth-first search (DFS) - Binary search tree - recursion

- n = number of nodes
- time complexity: $O(n)$
- space complexity: $O(n)$

## Second approach - Depth-first search (DFS) - Binary search tree - iterative

- n = number of nodes
- time complexity: $O(n)$
- space complexity: $O(n)$





