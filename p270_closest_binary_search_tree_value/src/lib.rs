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
                q.push_back(new_node);
            }
        }
    }
    Some(tree)
}

fn from_bt_to_vec(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<Option<i32>> {
    let mut result: Vec<Option<i32>> = vec![];
    let mut q = VecDeque::new();
    if let Some(node) = root {
        result.push(Some(node.borrow().val));
        q.push_back(Rc::clone(&node));
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
    pub fn closest_value_bs(root: Option<Rc<RefCell<TreeNode>>>, target: f64) -> i32 {
        let mut root_ptr = None;
        let mut closest = 0;
        if let Some(node) = root {
            closest = node.borrow().val;
            root_ptr = Some(Rc::clone(&node));
        }
        while let Some(node) = root_ptr.as_ref().map(Rc::clone) {
            let node = node.borrow();
            let d1 = (target - closest as f64).abs();
            let d2 = (node.val as f64 - target).abs();
            if d1 > d2 {
                closest = node.val
            } else if d1 == d2 {
                closest = closest.min(node.val)
            };
            root_ptr = if target < node.val as f64 {
                node.left.as_ref().map(Rc::clone)
            } else {
                node.right.as_ref().map(Rc::clone)
            };
        }
        closest
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
        let vec = vec![Some(4), Some(2), Some(5), Some(1), Some(3)];
        let root = from_vec_to_bt(&vec);
        let target = 3.714286;
        let result1 = Solution::closest_value_bs(root, target);
        assert_eq!(result1, 4);
    }

    #[test]
    fn case_02() {
        let vec = vec![Some(1)];
        let root = from_vec_to_bt(&vec);
        let target = 4.428571;
        let result1 = Solution::closest_value_bs(root, target);
        assert_eq!(result1, 1);
    }

    #[test]
    fn case_03() {
        let vec = vec![Some(1), None, Some(2)];
        let root = from_vec_to_bt(&vec);
        let target = 3.428571;
        let result1 = Solution::closest_value_bs(root, target);
        assert_eq!(result1, 2);
    }

    #[test]
    fn case_04() {
        let vec = vec![Some(4), Some(2), Some(5), Some(1), Some(3)];
        let root = from_vec_to_bt(&vec);
        let target = 3.5;
        let result1 = Solution::closest_value_bs(root, target);
        assert_eq!(result1, 3);
    }
}
