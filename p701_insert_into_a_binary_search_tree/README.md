# p701_insert_into_a_binary_search_tree
[https://leetcode.com/problems/insert-into-a-binary-search-tree/](https://leetcode.com/problems/insert-into-a-binary-search-tree/)

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
    pub fn insert_into_bst(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        
    }
}
```
## First approach - Depth-first search (DFS) - Binary search tree - recursion

- n = number of nodes
- h = tree high
- time complexity: $O(h) =$ $O(\log n)$ (avg case) $= O(n)$ (worst case)
- space complexity: $O(h) =$ $O(\log n)$ (avg case) $= O(n)$ (worst case)

## Second approach - Depth-first search (DFS) - Binary search tree - iterative

- n = number of nodes
- h = tree high
- time complexity: $O(h) =$ $O(\log n)$ (avg case) $= O(n)$ (worst case)
- space complexity: $O(1)$





