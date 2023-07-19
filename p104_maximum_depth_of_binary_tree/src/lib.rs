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

struct Solution;
impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            None => 0,
            Some(node) => {
                let node = node.borrow();
                let left = node.left.as_ref().map(|rc| Rc::clone(rc));
                let right = node.right.as_ref().map(Rc::clone);
                let left_height = Solution::max_depth(left);
                let right_height = Solution::max_depth(right);
                std::cmp::max(left_height, right_height) + 1
            }
        }
    }

    pub fn max_depth_tail(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }

        fn next_max_depth(q: &mut VecDeque<(Rc<RefCell<TreeNode>>, i32)>, max_depth: &mut i32) {
            while let Some((next_node, next_level)) = q.pop_front() {
                *max_depth = (*max_depth).max(next_level);

                let next_level = next_level + 1;
                let next_node = next_node.borrow();

                if let Some(left) = &next_node.left {
                    q.push_back((Rc::clone(left), next_level));
                }
                if let Some(right) = &next_node.right {
                    q.push_back((Rc::clone(right), next_level));
                }
            }
        }

        let mut q: VecDeque<(Rc<RefCell<TreeNode>>, i32)> = VecDeque::new();
        let mut max_depth = 0;

        q.push_back((Rc::clone(root.as_ref().unwrap()), 1));

        next_max_depth(&mut q, &mut max_depth);

        max_depth
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
        let result1 = Solution::max_depth(root.clone());
        let result2 = Solution::max_depth_tail(root.clone());
        assert_eq!(result1, 3);
        assert_eq!(result2, 3);
    }

    #[test]
    fn case_02() {
        let vec = vec![Some(1), None, Some(2)];
        let root = from_vec_to_bt(&vec);
        let result1 = Solution::max_depth(root.clone());
        let result2 = Solution::max_depth_tail(root.clone());
        assert_eq!(result1, 2);
        assert_eq!(result2, 2);
    }
}

fn from_vec_to_bt(vec: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
    let tree = Rc::new(RefCell::new(TreeNode::new(vec.get(0)?.unwrap())));
    let mut q = VecDeque::from([Rc::clone(&tree)]);
    let mut i = 0;

    while let Some(node) = q.pop_front() {
        let mut node = node.borrow_mut();
        i += 1;
        if let Some(&Some(val)) = vec.get(i) {
            let new_node = Rc::new(RefCell::new(TreeNode::new(val)));
            node.left = Some(Rc::clone(&new_node));
            q.push_back(new_node);
        }
        i += 1;
        if let Some(&Some(val)) = vec.get(i) {
            let new_node = Rc::new(RefCell::new(TreeNode::new(val)));
            node.right = Some(Rc::clone(&new_node));
            q.push_back(new_node);
        }
    }

    Some(tree)
}

fn from_bt_to_vec(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<Option<i32>> {
    let mut result = Vec::new();
    let mut q = VecDeque::new();

    if let Some(root) = root {
        q.push_back(Rc::clone(root));
        result.push(Some(root.borrow().val));
    }

    while let Some(node) = q.pop_front() {
        let node = node.borrow();

        if let Some(left) = &node.left {
            q.push_back(Rc::clone(left));
            result.push(Some(left.borrow().val));
        } else {
            result.push(None);
        }

        if let Some(right) = &node.right {
            q.push_back(Rc::clone(right));
            result.push(Some(right.borrow().val));
        } else {
            result.push(None)
        }
    }

    while result[result.len() - 1].is_none() {
        result.pop();
    }

    result
}
