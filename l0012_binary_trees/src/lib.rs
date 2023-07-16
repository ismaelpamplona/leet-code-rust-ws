use std::cell::RefCell;
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
            Self::dfs(&borrowed.left); // 1
            Self::dfs(&borrowed.right); // 2
        }
    }

    pub fn preorder_dfs(root: &Option<Rc<RefCell<TreeNode>>>) {
        if let Some(node) = root {
            let borrowed = node.borrow();
            println!("{:?}", &borrowed.val); // preorder logic
            Self::preorder_dfs(&borrowed.left); // 1
            Self::preorder_dfs(&borrowed.right); // 2
        }
    }

    pub fn inorder_dfs(root: &Option<Rc<RefCell<TreeNode>>>) {
        if let Some(node) = root {
            let borrowed = node.borrow();
            Self::inorder_dfs(&borrowed.left); // 1
            println!("{:?}", &borrowed.val); // inorder logic
            Self::inorder_dfs(&borrowed.right); // 2
        }
    }

    pub fn postorder_dfs(root: &Option<Rc<RefCell<TreeNode>>>) {
        if let Some(node) = root {
            let borrowed = node.borrow();
            Self::postorder_dfs(&borrowed.left); // 1
            Self::postorder_dfs(&borrowed.right); // 2
            println!("{:?}", &borrowed.val); // postorder logic
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefMut, collections::HashMap, rc::Weak};

    use super::*;

    #[test]
    fn case_rc() {
        let test = Rc::new(1);
        let down: Weak<i32> = Rc::downgrade(&test);
        let up: Option<Rc<i32>> = down.upgrade();

        let foo = Rc::new(vec![1.0, 2.0, 3.0]);
        let a = foo.clone();
        let b = Rc::clone(&foo);
    }

    #[test]
    fn case_ref_cell() {
        let shared_map: Rc<RefCell<_>> = Rc::new(RefCell::new(HashMap::new()));
        // Create a new block to limit the scope of the dynamic borrow
        {
            let mut map: RefMut<'_, _> = shared_map.borrow_mut();
            map.insert("africa", 92388);
            map.insert("kyoto", 11837);
            map.insert("piccadilly", 11826);
            map.insert("marbles", 38);
            let total_map: i32 = map.values().sum();
            assert_eq!(total_map, 116089);
        }
        let total_shared: i32 = shared_map.borrow().values().sum();
        assert_eq!(total_shared, 116089);
    }

    #[test]
    fn case_dfs() {
        let node6 = Rc::new(RefCell::new(TreeNode::new(6)));
        let node5 = Rc::new(RefCell::new(TreeNode::new(5)));
        let node4 = Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: None,
            right: Some(node6),
        }));
        let node3 = Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: None,
            right: None,
        }));
        let node2 = Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: None,
            right: Some(node5),
        }));
        let node1 = Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(node3),
            right: Some(node4),
        }));
        let root = Rc::new(RefCell::new(TreeNode {
            val: 0,
            left: Some(node1),
            right: Some(node2),
        }));
        Solution::dfs(&Some(root.clone()));
        Solution::preorder_dfs(&Some(root.clone()));
        Solution::inorder_dfs(&Some(root.clone()));
        Solution::postorder_dfs(&Some(root.clone()));
    }
}
