use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

// Definition for a binary tree node.
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

struct Solution;
impl Solution {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let left_2 = Rc::new(RefCell::new(TreeNode::new(3)));
        let right_2 = Rc::new(RefCell::new(TreeNode::new(1)));
        let left_7 = Rc::new(RefCell::new(TreeNode::new(9)));
        let right_7 = Rc::new(RefCell::new(TreeNode::new(6)));
        let left_4 = Rc::new(RefCell::new(TreeNode {
            val: 7,
            left: Some(left_7.clone()),
            right: Some(right_7.clone()),
        }));
        let right_4 = Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(left_2.clone()),
            right: Some(right_2.clone()),
        }));

        let root = Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(left_4.clone()),
            right: Some(right_4.clone()),
        }));
        let tn_vec = vec![4, 2, 7, 1, 3, 6, 9];
        let tn = from_vec_to_tn(&tn_vec, 0);
        let result = Solution::invert_tree_rec(tn);
        assert_eq!(Some(root), result);
    }

    #[test]
    fn case_2() {
        let left_2 = Rc::new(RefCell::new(TreeNode::new(3)));
        let right_2 = Rc::new(RefCell::new(TreeNode::new(1)));
        let root = Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(left_2.clone()),
            right: Some(right_2.clone()),
        }));
        let tn_vec = vec![2, 1, 3];
        let tn = from_vec_to_tn(&tn_vec, 0);
        let result = Solution::invert_tree_rec(tn);
        assert_eq!(Some(root), result);
    }

    #[test]
    fn case_3() {
        let tn_vec = vec![];
        let tn = from_vec_to_tn(&tn_vec, 0);
        let result = Solution::invert_tree_rec(tn);
        assert_eq!(None, result);
    }

    #[test]
    fn case_from_tn_to_vec_1() {
        let left_2 = Rc::new(RefCell::new(TreeNode::new(1)));
        let right_2 = Rc::new(RefCell::new(TreeNode::new(3)));
        let left_7 = Rc::new(RefCell::new(TreeNode::new(6)));
        let right_7 = Rc::new(RefCell::new(TreeNode::new(9)));
        let left_4 = Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(left_2.clone()),
            right: Some(right_2.clone()),
        }));
        let right_4 = Rc::new(RefCell::new(TreeNode {
            val: 7,
            left: Some(left_7.clone()),
            right: Some(right_7.clone()),
        }));

        let root = Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(left_4.clone()),
            right: Some(right_4.clone()),
        }));
        let goal_result = vec![4, 2, 7, 1, 3, 6, 9];
        let target_result = from_tn_to_vec(Some(root.clone()));
        let target_result_beta = from_tn_to_vec_beta(Some(root.clone()));
        assert_eq!(goal_result, target_result);
        assert_eq!(goal_result, target_result_beta);
    }

    #[test]
    fn case_from_vec_to_tn_1() {
        let left_2 = Rc::new(RefCell::new(TreeNode::new(1)));
        let right_2 = Rc::new(RefCell::new(TreeNode::new(3)));
        let left_7 = Rc::new(RefCell::new(TreeNode::new(6)));
        let right_7 = Rc::new(RefCell::new(TreeNode::new(9)));
        let left_4 = Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(left_2.clone()),
            right: Some(right_2.clone()),
        }));
        let right_4 = Rc::new(RefCell::new(TreeNode {
            val: 7,
            left: Some(left_7.clone()),
            right: Some(right_7.clone()),
        }));

        let root = Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(left_4.clone()),
            right: Some(right_4.clone()),
        }));
        let tn_vec = vec![4, 2, 7, 1, 3, 6, 9];
        let result = from_vec_to_tn(&tn_vec, 0);
        assert_eq!(Some(root), result);
    }
}

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

fn from_tn_to_vec_beta(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];
    let mut queue: Vec<Rc<RefCell<TreeNode>>> = vec![];
    if let Some(root) = root {
        queue.push(root.clone());
    }
    while !queue.is_empty() {
        if let Some(node) = Some(queue.remove(0)) {
            result.push(node.borrow().val);

            if let Some(left) = node.borrow().left.clone() {
                queue.push(left);
            }
            if let Some(right) = node.borrow().right.clone() {
                queue.push(right);
            }
        }
    }
    result
}
