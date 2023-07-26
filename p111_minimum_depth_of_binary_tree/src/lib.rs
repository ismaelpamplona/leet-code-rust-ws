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
                let new_node = &Rc::new(RefCell::new(TreeNode::new(val)));
                node[j] = Some(Rc::clone(new_node));
                q.push_back(Rc::clone(new_node));
            }
        }
    }
    Some(tree)
}

fn from_bt_to_vec(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<Option<i32>> {
    let mut result = vec![];
    let mut q = VecDeque::new();
    if let Some(node) = root {
        q.push_back(Rc::clone(node));
        result.push(Some(node.borrow().val));
    }
    while let Some(node) = q.pop_front() {
        for i in 0..=1 {
            if let Some(leaf) = &node.borrow()[i] {
                result.push(Some(leaf.borrow().val));
                q.push_back(Rc::clone(leaf));
            } else {
                result.push(None);
            }
        }
    }

    while result[result.len() - 1].is_none() {
        result.pop();
    }

    result
}

struct Solution;
impl Solution {
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(node: Option<Rc<RefCell<TreeNode>>>) -> i32 {
            if let Some(node) = node {
                let left = node.borrow().left.as_ref().map(Rc::clone);
                let right = node.borrow().right.as_ref().map(Rc::clone);
                if node.borrow().left.is_none() {
                    return 1 + dfs(right);
                } else if node.borrow().right.is_none() {
                    return 1 + dfs(left);
                }
                return 1 + dfs(left).min(dfs(right));
            }
            0
        }
        dfs(root)
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
        let vec = vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)];
        let root = from_vec_to_bt(&vec);
        let result1 = Solution::min_depth(root.clone());
        assert_eq!(result1, 2);
    }

    #[test]
    fn case_02() {
        let vec = vec![
            Some(2),
            None,
            Some(3),
            None,
            Some(4),
            None,
            Some(5),
            None,
            Some(6),
        ];
        let root = from_vec_to_bt(&vec);
        let result1 = Solution::min_depth(root.clone());
        assert_eq!(result1, 5);
    }
}
