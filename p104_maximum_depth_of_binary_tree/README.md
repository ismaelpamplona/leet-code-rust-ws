# p104_maximum_depth_of_binary_tree
[https://leetcode.com/problems/maximum-depth-of-binary-tree/](https://leetcode.com/problems/maximum-depth-of-binary-tree/)

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
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        
    }
}
```
## First approach - Depth-first search (DFS) (recursion)

- n = number of nodes
- time complexity: $O(n)$
- space complexity: $O(log(n))$

## Second approach - Depth-first search (DFS) (tail recursion)

- n = number of nodes
- time complexity: $O(n)$
- space complexity: $O(2^{(log(n−1))}) = O(n/2) = O(n)$

## Third approach - Depth-first search (DFS) (preorder iterative)

- n = number of nodes
- time complexity: $O(n)$
- space complexity: $O(logn)$

