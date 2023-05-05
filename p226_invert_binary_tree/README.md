# p226_invert_binary_tree
[https://leetcode.com/problems/invert-binary-tree](https://leetcode.com/problems/invert-binary-tree)

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
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        
    }
}
```
So, at this point I know:
1. the parameter type;
2. the return type; and 
3. that LeetCode tests uses a vector as input.

## Auxiliary Functions

To solve this problem, I need to write two auxiliary functions:

### 1. Convert TreeNode to a vector:

To convert a TreeNode to a vector, I used **Breadth Frst Search - BFS** concept inspired by the following video: [Breadth First Search Algorithm | Shortest Path | Graph Theory](https://www.youtube.com/watch?v=oDqjPvD54Ss).

```Rust
fn from_tn_to_vec(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut result = Vec::new();
    let mut queue = VecDeque::new();

    if let Some(root) = root {
        queue.push_back(root.clone());
    }

    while !queue.is_empty() {
        if let Some(node) = queue.pop_front() {
            result.push(node.borrow().val);

            if let Some(left) = node.borrow().left.clone() {
                queue.push_back(left);
            }

            if let Some(right) = node.borrow().right.clone() {
                queue.push_back(right);
            }
        }
    }
    result
}
```

### 2. Covert a vector to TreeNode:

#### Finding the math relation of the node positions:

Left:
```
0 -> 1
1 -> 3
3 -> 7
```

Right:
```
0 -> 2 (left + 1)
2 -> 4 (left + 1)
4 -> 8 (left + 1)
```

$$
    y_l = ax_l +b \\
    1 = a * 0 + b \\
    b = 1

    3 = a * 1 + 1 \\
    a = 2 \\
    
    y_l = 2 * x_l + 1 \\
    y_r = y_l + 1
$$


So, the functions receives two arguments: the vector `vec` and node position `n`.

```Rust
fn from_vec_to_tn(vec: &Vec<i32>, n: usize) -> Option<Rc<RefCell<TreeNode>>> {
    let mut tn = None;

    if vec.is_empty() {
        return None;
    } else if n < vec.len() {
        tn = Some(Rc::new(RefCell::new(TreeNode {
            val: vec[n],
            left: from_vec_to_tn(vec, 2 * n + 1),
            right: from_vec_to_tn(vec, 2 * n + 2),
        })));
    }
    tn
}
```

## First approach - Recursive

My first approach was to use recursion 

```Rust
 pub fn invert_tree_rec(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
     let mut clone = root.clone();
     if let None = root {
         return None;
     } else if let Some(node) = clone {
         clone = Some(Rc::new(RefCell::new(TreeNode {
             val: node.borrow().val,
             left: Solution::invert_tree_rec(node.borrow().right.clone()),
             right: Solution::invert_tree_rec(node.borrow().left.clone()),
         })));
     }
     clone
 }
```
- n = number of nodes
- time complexity: O(n)
- space complexity: O(n)

## Second approach - Iterative

After seeing the official solution, I tried to implement the iterative solution.

```Rust
  pub fn invert_tree_it(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
      let mut queue = VecDeque::new();
      if let Some(node) = root.clone() {
          queue.push_back(node);
      }
      while !queue.is_empty() {
          if let Some(node) = queue.pop_front() {
              let mut borrowed_node = node.borrow_mut();
              let left = borrowed_node.left.take();
              let right = borrowed_node.right.take();
              borrowed_node.left = right;
              borrowed_node.right = left;
              if let Some(left) = borrowed_node.left.clone() {
                  queue.push_back(left);
              }
              if let Some(right) = borrowed_node.right.clone() {
                  queue.push_back(right);
              }
          }
      }
      root
  }
```


- n = number of nodes
- time complexity: O(n)
- space complexity: O(n)

