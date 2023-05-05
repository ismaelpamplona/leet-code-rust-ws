use std::cell::RefCell;
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

pub fn check_tree(_root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let mut left_val = 0;
    let mut right_val = 0;

    if let Some(node) = _root {
        let val = node.borrow().val;
        if val < -100 || val > 100 {
            return false;
        }
        if let Some(left) = node.borrow().left.clone() {
            left_val = left.borrow().val;
        }
        if let Some(right) = node.borrow().right.clone() {
            right_val = right.borrow().val;
        }

        if left_val + right_val == val {
            return true;
        }
    }

    false
}

// pub fn check_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
//     let root_node = root.as_ref().unwrap().borrow();
//     let left_node = root_node.left.as_ref().unwrap().borrow();
//     let right_node = root_node.right.as_ref().unwrap().borrow();

//     root_node.val == left_node.val + right_node.val
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let left = Rc::new(RefCell::new(TreeNode::new(4)));
        let right = Rc::new(RefCell::new(TreeNode::new(6)));
        let root = Rc::new(RefCell::new(TreeNode {
            val: 10,
            left: Some(left),
            right: Some(right),
        }));
        let result = check_tree(Some(root));
        assert_eq!(result, true);
    }

    #[test]
    fn case_2() {
        let left = Rc::new(RefCell::new(TreeNode::new(3)));
        let right = Rc::new(RefCell::new(TreeNode::new(1)));
        let root = Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: Some(left),
            right: Some(right),
        }));
        let result = check_tree(Some(root));
        assert_eq!(result, false);
    }

    #[test]
    fn case_3() {
        let left = Rc::new(RefCell::new(TreeNode::new(3)));
        let root = Rc::new(RefCell::new(TreeNode {
            val: 10,
            left: Some(left),
            right: None,
        }));
        let result = check_tree(Some(root));
        assert_eq!(result, false);
    }

    #[test]
    fn case_4() {
        let root = Rc::new(RefCell::new(TreeNode {
            val: 10,
            left: None,
            right: None,
        }));
        let result = check_tree(Some(root));
        assert_eq!(result, false);
    }

    #[test]
    fn case_5() {
        let left = Rc::new(RefCell::new(TreeNode::new(0)));
        let right = Rc::new(RefCell::new(TreeNode::new(0)));
        let root = Rc::new(RefCell::new(TreeNode {
            val: 0,
            left: Some(left),
            right: Some(right),
        }));
        let result = check_tree(Some(root));
        assert_eq!(result, true);
    }

    #[test]
    fn case_6() {
        let left = Rc::new(RefCell::new(TreeNode::new(0)));
        let root = Rc::new(RefCell::new(TreeNode {
            val: 0,
            left: Some(left),
            right: None,
        }));
        let result = check_tree(Some(root));
        assert_eq!(result, true);
    }

    #[test]
    fn case_7() {
        let root = Rc::new(RefCell::new(TreeNode {
            val: 0,
            left: None,
            right: None,
        }));
        let result = check_tree(Some(root));
        assert_eq!(result, true);
    }

    #[test]
    fn case_8() {
        let left = Rc::new(RefCell::new(TreeNode::new(100)));
        let right = Rc::new(RefCell::new(TreeNode::new(1)));
        let root = Rc::new(RefCell::new(TreeNode {
            val: 101,
            left: Some(left),
            right: Some(right),
        }));
        let result = check_tree(Some(root));
        assert_eq!(result, false);
    }
}
