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
                q.push_back(Rc::clone(&new_node));
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
        let node = node.borrow();
        for i in 0..=1 {
            if let Some(leaf) = &node[i] {
                result.push(Some(leaf.borrow().val));
                q.push_back(Rc::clone(leaf));
            } else {
                result.push(None);
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
    pub fn insert_into_bst_rec(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root {
            let mut borrowed = node.borrow_mut();
            if val > borrowed.val {
                borrowed.right = Self::insert_into_bst_rec(borrowed.right.take(), val);
            } else {
                borrowed.left = Self::insert_into_bst_rec(borrowed.left.take(), val)
            }
            return Some(Rc::clone(&node));
        }
        Some(Rc::new(RefCell::new(TreeNode::new(val))))
    }

    pub fn insert_into_bst_it(
        mut root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut cur_root = None;
        if let Some(node) = &root {
            cur_root = Some(Rc::clone(node));
        }
        while let Some(node) = cur_root.as_ref().map(Rc::clone) {
            let mut borrowed = node.borrow_mut();
            if val > borrowed.val {
                if borrowed.right.is_none() {
                    borrowed.right = Some(Rc::new(RefCell::new(TreeNode::new(val))));
                    return root;
                } else {
                    cur_root = borrowed.right.as_ref().map(Rc::clone);
                }
            } else {
                if borrowed.left.is_none() {
                    borrowed.left = Some(Rc::new(RefCell::new(TreeNode::new(val))));
                    return root;
                } else {
                    cur_root = borrowed.left.as_ref().map(Rc::clone);
                }
            }
        }
        Some(Rc::new(RefCell::new(TreeNode::new(val))))
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
        let vec = vec![Some(4), Some(2), Some(7), Some(1), Some(3)];
        let root = from_vec_to_bt(&vec);
        let result1 = Solution::insert_into_bst_rec(root.clone(), 5);
        let root = from_vec_to_bt(&vec);
        let result2 = Solution::insert_into_bst_it(root.clone(), 5);
        let expected = vec![Some(4), Some(2), Some(7), Some(1), Some(3), Some(5)];
        assert_eq!(from_bt_to_vec(&result1), expected);
        assert_eq!(from_bt_to_vec(&result2), expected);
    }

    #[test]
    fn case_02() {
        let vec = vec![
            Some(40),
            Some(20),
            Some(60),
            Some(10),
            Some(30),
            Some(50),
            Some(70),
        ];
        let root = from_vec_to_bt(&vec);
        let result1 = Solution::insert_into_bst_rec(root.clone(), 25);
        let root = from_vec_to_bt(&vec);
        let result2 = Solution::insert_into_bst_it(root.clone(), 25);
        let expected = vec![
            Some(40),
            Some(20),
            Some(60),
            Some(10),
            Some(30),
            Some(50),
            Some(70),
            None,
            None,
            Some(25),
        ];
        assert_eq!(from_bt_to_vec(&result1), expected);
        assert_eq!(from_bt_to_vec(&result2), expected);
    }

    #[test]
    fn case_03() {
        let vec = vec![
            Some(4),
            Some(2),
            Some(7),
            Some(1),
            Some(3),
            None,
            None,
            None,
            None,
            None,
            None,
        ];
        let root = from_vec_to_bt(&vec);
        let result1 = Solution::insert_into_bst_rec(root.clone(), 5);
        let root = from_vec_to_bt(&vec);
        let result2 = Solution::insert_into_bst_it(root.clone(), 5);
        let expected = vec![Some(4), Some(2), Some(7), Some(1), Some(3), Some(5)];
        assert_eq!(from_bt_to_vec(&result1), expected);
        assert_eq!(from_bt_to_vec(&result2), expected);
    }

    #[test]
    fn case_04() {
        let vec = vec![];
        let root = from_vec_to_bt(&vec);
        let result1 = Solution::insert_into_bst_rec(root.clone(), 5);
        let root = from_vec_to_bt(&vec);
        let result2 = Solution::insert_into_bst_it(root.clone(), 5);
        let expected = vec![Some(5)];
        assert_eq!(from_bt_to_vec(&result1), expected);
        assert_eq!(from_bt_to_vec(&result2), expected);
    }
}
