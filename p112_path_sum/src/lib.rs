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
                q.push_back(new_node)
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
        q.push_back(Rc::clone(node));
    }

    while let Some(node) = q.pop_front() {
        let node = node.borrow();
        for i in 0..=1 {
            if let Some(node) = &node[i] {
                result.push(Some(node.borrow().val));
                q.push_back(Rc::clone(node));
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
    pub fn has_path_sum_inside_fn_dfs(
        root: Option<Rc<RefCell<TreeNode>>>,
        target_sum: i32,
    ) -> bool {
        fn dfs(node: Option<Rc<RefCell<TreeNode>>>, mut curr: i32, target_sum: i32) -> bool {
            if let Some(nd) = node {
                let node = nd.borrow_mut();
                if node.left.is_none() && node.right.is_none() {
                    return (curr + node.val) == target_sum;
                }

                curr += node.val;
                let left_ptr = node.left.as_ref().map(Rc::clone);
                let left_return = dfs(left_ptr, curr, target_sum);
                let right_ptr = node.right.as_ref().map(Rc::clone);
                let right_return = dfs(right_ptr, curr, target_sum);
                left_return || right_return
            } else {
                false
            }
        }
        dfs(root, 0, target_sum)
    }

    pub fn has_path_sum_rec(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        let mut curr = target_sum;
        if let Some(node) = root {
            let node = node.borrow();
            curr -= node.val;
            if node.left.is_none() && node.right.is_none() {
                return curr == 0;
            }
            let left_ptr = node.left.as_ref().map(Rc::clone);
            let left_return = Self::has_path_sum_rec(left_ptr, curr);
            let right_ptr = node.right.as_ref().map(Rc::clone);
            let right_return = Self::has_path_sum_rec(right_ptr, curr);
            left_return || right_return
        } else {
            false
        }
    }

    pub fn has_path_sum_it(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        if root.is_none() {
            return false;
        }
        let mut stack: Vec<(Rc<RefCell<TreeNode>>, i32)> = vec![];

        if let Some(node) = root {
            stack.push((Rc::clone(&node), target_sum - node.borrow().val));
        }

        while !stack.is_empty() {
            let (node, curr) = stack.pop().unwrap();
            let nd = node.borrow();
            if nd.left.is_none() && nd.right.is_none() && curr == 0 {
                return true;
            }

            for i in 0..=1 {
                if let Some(leaf) = &nd[i] {
                    stack.push((Rc::clone(leaf), curr - leaf.borrow().val));
                }
            }
        }

        false
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
        let result1 = Solution::has_path_sum_inside_fn_dfs(root.clone(), 22);
        let result2 = Solution::has_path_sum_rec(root.clone(), 22);
        let result3 = Solution::has_path_sum_it(root.clone(), 22);
        assert_eq!(result1, true);
        assert_eq!(result2, true);
        assert_eq!(result3, true);
    }

    #[test]
    fn case_02() {
        let vec = vec![Some(1), Some(2), Some(3)];
        let root = from_vec_to_bt(&vec);
        let result1 = Solution::has_path_sum_inside_fn_dfs(root.clone(), 5);
        let result2 = Solution::has_path_sum_rec(root.clone(), 5);
        let result3 = Solution::has_path_sum_it(root.clone(), 5);
        assert_eq!(result1, false);
        assert_eq!(result2, false);
        assert_eq!(result3, false);
    }

    #[test]
    fn case_03() {
        let vec = vec![];
        let root = from_vec_to_bt(&vec);
        let result1 = Solution::has_path_sum_inside_fn_dfs(root.clone(), 5);
        let result2 = Solution::has_path_sum_rec(root.clone(), 5);
        let result3 = Solution::has_path_sum_it(root.clone(), 5);
        assert_eq!(result1, false);
        assert_eq!(result2, false);
        assert_eq!(result3, false);
    }
}
