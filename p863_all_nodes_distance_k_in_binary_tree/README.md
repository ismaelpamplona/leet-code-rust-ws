# p863_all_nodes_distance_k_in_binary_tree
[https://leetcode.com/problems/all-nodes-distance-k-in-binary-tree/](https://leetcode.com/problems/all-nodes-distance-k-in-binary-tree/)

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
    pub fn distance_k(root: Option<Rc<RefCell<TreeNode>>>, target: Option<Rc<RefCell<TreeNode>>>, k: i32) -> Vec<i32> {
        
    }
}
```

## First approach - Implementing Parent Pointers with HashMap (DFS + BFS)
- $n$: number of nodes
- time complexity: $O(n)$
- space complexity: $O(n)$

## Second approach - Implementing Parent Pointers with a new Graph Struct (DFS + BFS)
- $n$: number of nodes
- time complexity: $O(n)$
- space complexity: $O(n)$


