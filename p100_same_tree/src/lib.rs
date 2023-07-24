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
        for j in 0..2 {
            i += 1;
            if let Some(&Some(val)) = vec.get(i) {
                let new_node = Rc::new(RefCell::new(TreeNode::new(val)));
                node[j] = Some(Rc::clone(&new_node));
                q.push_back(new_node);
            }
        }
    }
    Some(tree)
}

fn from_bt_to_vec(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<Option<i32>> {
    let mut result = vec![];
    let mut q = VecDeque::new();
    if let Some(node) = root {
        result.push(Some(node.borrow().val));
        q.push_back(Rc::clone(node));
    }

    while let Some(node) = q.pop_front() {
        let node = node.borrow();
        for i in 0..2 {
            if let Some(leaf) = &node[i] {
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
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if p.is_none() && q.is_none() {
            return true;
        }
        if let (Some(p_node), Some(q_node)) = (p, q) {
            let p_node = p_node.borrow();
            let q_node = q_node.borrow();
            if p_node.val == q_node.val {
                let p_left = p_node.left.as_ref().map(Rc::clone);
                let q_left = q_node.left.as_ref().map(Rc::clone);
                let left = Self::is_same_tree(p_left, q_left);
                let p_right = p_node.right.as_ref().map(Rc::clone);
                let q_right = q_node.right.as_ref().map(Rc::clone);
                let right = Self::is_same_tree(p_right, q_right);
                return left && right;
            }
        }
        false
    }

    pub fn is_same_tree_concise(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (p, q) {
            (None, None) => true,
            (Some(p_node), Some(q_node)) => {
                let p_node = p_node.borrow();
                let q_node = q_node.borrow();
                p_node.val == q_node.val
                    && Self::is_same_tree(
                        p_node.left.as_ref().map(Rc::clone),
                        q_node.left.as_ref().map(Rc::clone),
                    )
                    && Self::is_same_tree(
                        p_node.right.as_ref().map(Rc::clone),
                        q_node.right.as_ref().map(Rc::clone),
                    )
            }
            _ => false,
        }
    }

    pub fn is_same_tree_it(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        let mut stack = vec![(p, q)];
        while !stack.is_empty() {
            match stack.pop() {
                Some((None, None)) => continue,
                Some((Some(p_node), Some(q_node))) => {
                    if p_node.borrow().val != q_node.borrow().val {
                        return false;
                    }
                    stack.push((
                        p_node.borrow().left.as_ref().map(Rc::clone),
                        q_node.borrow().left.as_ref().map(Rc::clone),
                    ));
                    stack.push((
                        p_node.borrow().right.as_ref().map(Rc::clone),
                        q_node.borrow().right.as_ref().map(Rc::clone),
                    ));
                }
                _ => return false,
            };
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
        let p_vec = vec![Some(1), Some(2), Some(3)];
        let q_vec = vec![Some(1), Some(2), Some(3)];
        let p_tree = from_vec_to_bt(&p_vec);
        let q_tree = from_vec_to_bt(&q_vec);
        let result1 = Solution::is_same_tree(p_tree.clone(), q_tree.clone());
        let result2 = Solution::is_same_tree_concise(p_tree.clone(), q_tree.clone());
        let result3 = Solution::is_same_tree_it(p_tree.clone(), q_tree.clone());
        assert_eq!(result1, true);
        assert_eq!(result2, true);
        assert_eq!(result3, true);
    }

    #[test]
    fn case_02() {
        let p_vec = vec![Some(1), Some(2)];
        let q_vec = vec![Some(1), None, Some(2)];
        let p_tree = from_vec_to_bt(&p_vec);
        let q_tree = from_vec_to_bt(&q_vec);
        let result1 = Solution::is_same_tree(p_tree.clone(), q_tree.clone());
        let result2 = Solution::is_same_tree_concise(p_tree.clone(), q_tree.clone());
        let result3 = Solution::is_same_tree_it(p_tree.clone(), q_tree.clone());
        assert_eq!(result1, false);
        assert_eq!(result2, false);
        assert_eq!(result3, false);
    }

    #[test]
    fn case_03() {
        let p_vec = vec![Some(1), Some(2), Some(1)];
        let q_vec = vec![Some(1), Some(1), Some(2)];
        let p_tree = from_vec_to_bt(&p_vec);
        let q_tree = from_vec_to_bt(&q_vec);
        let result1 = Solution::is_same_tree(p_tree.clone(), q_tree.clone());
        let result2 = Solution::is_same_tree_concise(p_tree.clone(), q_tree.clone());
        let result3 = Solution::is_same_tree_it(p_tree.clone(), q_tree.clone());
        assert_eq!(result1, false);
        assert_eq!(result2, false);
        assert_eq!(result3, false);
    }
}
