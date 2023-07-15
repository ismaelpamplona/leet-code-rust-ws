use std::cell::RefCell;
use std::ops::Deref;
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

struct Solution;
impl Solution {
    pub fn dfs(root: &Option<Rc<RefCell<TreeNode>>>) {
        if let Some(node) = root {
            let borrowed = node.borrow();
            let left = &borrowed.left;
            if let Some(left_rc) = left.as_ref() {
                println!("{:?}", left_rc.borrow().val)
            }
            Solution::dfs(left);
            let right = &borrowed.right;
            Solution::dfs(right);
            if let Some(right_rc) = right.as_ref() {
                println!("{:?}", right_rc.borrow().val)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_dfs() {
        let a_left_left = Rc::new(RefCell::new(TreeNode::new(6)));
        let a_left = Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(a_left_left.clone()),
            right: None,
        }));
        let a_right = Rc::new(RefCell::new(TreeNode::new(4)));
        let root_a = Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(a_left.clone()),
            right: Some(a_right.clone()),
        }));
        let root = Rc::new(RefCell::new(TreeNode {
            val: 0,
            left: Some(root_a.clone()),
            right: None,
        }));
        Solution::dfs(&Some(root));
    }
}
