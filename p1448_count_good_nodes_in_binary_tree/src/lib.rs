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
    pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>, mut higher: i32) -> i32 {
            if let Some(node) = root {
                let node = node.borrow();
                higher = higher.max(node.val);
                let left = dfs(node.left.as_ref().map(Rc::clone), higher);
                let right = dfs(node.right.as_ref().map(Rc::clone), higher);
                let mut ans = left + right;
                if node.val >= higher {
                    ans += 1;
                }
                return ans;
            }
            0
        }

        dfs(root, i32::MIN)
    }

    pub fn good_nodes_it(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut stack = vec![];
        if let Some(node) = root {
            stack.push((Rc::clone(&node), i32::MIN));
        } else {
            return 0;
        }
        let mut ans = 0;
        while !stack.is_empty() {
            if let Some((node, higher)) = stack.pop() {
                let node = node.borrow();
                if node.val >= higher {
                    ans += 1;
                }
                for i in 0..=1 {
                    if let Some(leaf) = &node[i] {
                        stack.push((Rc::clone(leaf), higher.max(node.val)));
                    }
                }
            }
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
        let vec = vec![Some(3), Some(1), Some(4), Some(3), None, Some(1), Some(5)];
        let root = from_vec_to_bt(&vec);
        let result1 = Solution::good_nodes(root.clone());
        let result2 = Solution::good_nodes_it(root.clone());
        assert_eq!(result1, 4);
        assert_eq!(result2, 4);
    }

    #[test]
    fn case_02() {
        let vec = vec![Some(3), Some(3), None, Some(4), Some(2)];
        let root = from_vec_to_bt(&vec);
        let result1 = Solution::good_nodes(root.clone());
        let result2 = Solution::good_nodes_it(root.clone());
        assert_eq!(result1, 3);
        assert_eq!(result2, 3);
    }

    #[test]
    fn case_03() {
        let vec = vec![Some(1)];
        let root = from_vec_to_bt(&vec);
        let result1 = Solution::good_nodes(root.clone());
        let result2 = Solution::good_nodes_it(root.clone());
        assert_eq!(result1, 1);
        assert_eq!(result2, 1);
    }

    #[test]
    fn case_04() {
        let vec = vec![
            Some(5),
            Some(4),
            Some(8),
            Some(11),
            None,
            Some(13),
            Some(4),
            Some(7),
            Some(2),
            None,
            None,
            None,
            Some(1),
        ];
        let root = from_vec_to_bt(&vec);
        let result1 = Solution::good_nodes(root.clone());
        let result2 = Solution::good_nodes_it(root.clone());
        assert_eq!(result1, 4);
        assert_eq!(result2, 4);
    }
}
