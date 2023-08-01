use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

use std::ops::{Index, IndexMut};
impl Index<usize> for TreeNode {
    type Output = Option<Rc<RefCell<TreeNode>>>;
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.left,
            1 => &self.right,
            n => panic!("Invalid TreeNode index: {}", n),
        }
    }
}

impl IndexMut<usize> for TreeNode {
    fn index_mut(&mut self, index: usize) -> &mut Option<Rc<RefCell<TreeNode>>> {
        match index {
            0 => &mut self.left,
            1 => &mut self.right,
            n => panic!("Invalid TreeNode index: {}", n),
        }
    }
}

fn from_vec_to_bt(vec: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
    let tree = Rc::new(RefCell::new(TreeNode::new(vec.get(0)?.unwrap())));
    let mut q = VecDeque::from([Rc::clone(&tree)]);
    let mut i = 0;
    while let Some(node) = q.pop_front() {
        let mut node = node.borrow_mut();
        for j in 0..=1 {
            i += 1;
            if let Some(&Some(val)) = vec.get(i) {
                let new_node = Rc::new(RefCell::new(TreeNode::new(val)));
                node[j] = Some(Rc::clone(&new_node));
                q.push_back(Rc::clone(&new_node))
            }
        }
    }
    Some(tree)
}

fn from_bt_to_vec(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<Option<i32>> {
    let mut result = vec![];
    let mut q = VecDeque::new();
    if let Some(node) = root {
        q.push_back(Rc::clone(&node));
        result.push(Some(node.borrow().val));
    }
    while let Some(node) = q.pop_front() {
        let node = node.borrow();
        for i in 0..=1 {
            if let Some(leaf) = &node[i] {
                result.push(Some(leaf.borrow().val));
                q.push_back(Rc::clone(leaf));
            } else {
                result.push(None)
            }
        }
    }
    while let Some(None) = result.last() {
        result.pop();
    }
    result
}

struct Solution;
impl Solution {
    pub fn is_valid_bst_rec(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>, lower: i64, higher: i64) -> bool {
            if let Some(node) = root {
                let node = node.borrow();
                if !((node.val as i64) > lower && (node.val as i64) < higher) {
                    return false;
                }
                let left = dfs(node.left.as_ref().map(Rc::clone), lower, node.val as i64);
                let right = dfs(node.right.as_ref().map(Rc::clone), node.val as i64, higher);
                return left && right;
            }
            true
        }
        dfs(root, i64::MIN, i64::MAX)
    }

    pub fn is_valid_bst_it(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut stack = vec![];
        if let Some(node) = root {
            stack.push((Rc::clone(&node), i64::MIN, i64::MAX));
        }
        while let Some((node, lower, higher)) = stack.pop() {
            let node = node.borrow();
            if !((node.val as i64) > lower && (node.val as i64) < higher) {
                return false;
            }
            if let Some(left) = node.left.as_ref() {
                stack.push((Rc::clone(left), lower, node.val as i64));
            }
            if let Some(right) = node.right.as_ref() {
                stack.push((Rc::clone(right), node.val as i64, higher));
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_transformation() {
        let vec1: Vec<Option<i32>> = vec![Some(0), None, Some(1), None, Some(2), None, Some(3)];
        let bt_from_vec1: Option<Rc<RefCell<TreeNode>>> = from_vec_to_bt(&vec1);
        let vec2: Vec<Option<i32>> = vec![Some(0), Some(1), None, Some(2), None, Some(3)];
        let bt_from_vec2: Option<Rc<RefCell<TreeNode>>> = from_vec_to_bt(&vec2);
        let vec1_from_bt = from_bt_to_vec(&bt_from_vec1);
        let vec2_from_bt = from_bt_to_vec(&bt_from_vec2);
        assert_eq!(vec1, vec1_from_bt);
        assert_eq!(vec2, vec2_from_bt);
    }

    #[test]
    fn case_01() {
        let vec = vec![Some(2), Some(1), Some(3)];
        let root = from_vec_to_bt(&vec);
        let result1 = Solution::is_valid_bst_rec(root.clone());
        let result2 = Solution::is_valid_bst_it(root.clone());
        assert_eq!(result1, true);
        assert_eq!(result2, true);
    }

    #[test]
    fn case_02() {
        let vec = vec![Some(5), Some(1), Some(4), None, None, Some(3), Some(6)];
        let root = from_vec_to_bt(&vec);
        let result1 = Solution::is_valid_bst_rec(root.clone());
        let result2 = Solution::is_valid_bst_it(root.clone());
        assert_eq!(result1, false);
        assert_eq!(result2, false);
    }

    #[test]
    fn case_03() {
        let vec = vec![Some(2), Some(2), Some(2)];
        let root = from_vec_to_bt(&vec);
        let result1 = Solution::is_valid_bst_rec(root.clone());
        let result2 = Solution::is_valid_bst_it(root.clone());
        assert_eq!(result1, false);
        assert_eq!(result2, false);
    }

    #[test]
    fn case_04() {
        let vec = vec![Some(5), Some(4), Some(6), None, None, Some(3), Some(7)];
        let root = from_vec_to_bt(&vec);
        let result1 = Solution::is_valid_bst_rec(root.clone());
        let result2 = Solution::is_valid_bst_it(root.clone());
        assert_eq!(result1, false);
        assert_eq!(result2, false);
    }

    #[test]
    fn case_05() {
        let vec = vec![Some(2147483647)];
        let root = from_vec_to_bt(&vec);
        let result1 = Solution::is_valid_bst_rec(root.clone());
        let result2 = Solution::is_valid_bst_it(root.clone());
        assert_eq!(result1, true);
        assert_eq!(result2, true);
    }
}
