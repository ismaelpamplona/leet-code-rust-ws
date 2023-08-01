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
    let mut result: Vec<Option<i32>> = vec![];
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
    pub fn get_minimum_difference_rec(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>, values: &mut Vec<i32>) {
            if let Some(node) = root {
                let node = node.borrow();
                dfs(node.left.as_ref().map(Rc::clone), values);
                values.push(node.val);
                dfs(node.right.as_ref().map(Rc::clone), values);
            }
            return;
        }
        let mut values: Vec<i32> = vec![];
        dfs(root, &mut values);
        let mut ans = i32::MAX;
        for i in 1..values.len() {
            ans = ans.min(values[i] - values[i - 1]);
        }
        ans
    }

    pub fn get_minimum_difference_it(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut stack = Vec::new();
        let mut values = Vec::new();
        let mut cur = root;
        let mut ans = i32::MAX;
        while !stack.is_empty() || cur.is_some() {
            if let Some(node) = cur {
                stack.push(Rc::clone(&node));
                cur = node.borrow().left.as_ref().map(Rc::clone);
            } else if let Some(node) = stack.pop() {
                values.push(node.borrow().val);
                cur = node.borrow().right.as_ref().map(Rc::clone);
            }
        }
        for i in 1..values.len() {
            ans = ans.min(values[i] - values[i - 1]);
        }
        ans
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
        let vec = vec![Some(4), Some(2), Some(6), Some(1), Some(3)];
        let root = from_vec_to_bt(&vec);
        let result1 = Solution::get_minimum_difference_rec(root.clone());
        let result2 = Solution::get_minimum_difference_it(root.clone());
        assert_eq!(result1, 1);
        assert_eq!(result2, 1);
    }

    #[test]
    fn case_02() {
        let vec = vec![Some(1), Some(0), Some(48), None, None, Some(12), Some(49)];
        let root = from_vec_to_bt(&vec);
        let result1 = Solution::get_minimum_difference_rec(root.clone());
        let result2 = Solution::get_minimum_difference_it(root.clone());
        assert_eq!(result1, 1);
        assert_eq!(result2, 1);
    }
}
