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
                q.push_back(Rc::clone(&new_node));
                node[j] = Some(new_node);
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
                q.push_back(Rc::clone(leaf));
                result.push(Some(leaf.borrow().val));
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
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut result = vec![];
        let mut q = VecDeque::new();
        if let Some(node) = root {
            q.push_back(Rc::clone(&node));
        }
        while !q.is_empty() {
            let num_els_row = q.len();
            let mut level_items = vec![0; num_els_row];

            for i in 0..num_els_row {
                if let Some(node) = q.pop_front() {
                    let node = node.borrow();
                    if result.len() % 2 == 1 {
                        level_items[num_els_row - i - 1] = node.val;
                    } else {
                        level_items[i] = node.val;
                    }

                    for i in 0..=1 {
                        if let Some(leaf) = &node[i] {
                            q.push_back(Rc::clone(leaf));
                        }
                    }
                }
            }

            result.push(level_items);
        }
        result
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
        let result1 = Solution::zigzag_level_order(root);
        let expected = vec![vec![3], vec![20, 9], vec![15, 7]];
        assert_eq!(result1, expected);
    }

    #[test]
    fn case_02() {
        let vec = vec![Some(1)];
        let root = from_vec_to_bt(&vec);
        let result1 = Solution::zigzag_level_order(root);
        let expected = vec![vec![1]];
        assert_eq!(result1, expected);
    }

    #[test]
    fn case_03() {
        let vec = vec![];
        let root = from_vec_to_bt(&vec);
        let result1 = Solution::zigzag_level_order(root);
        let expected: Vec<Vec<i32>> = vec![];
        assert_eq!(result1, expected);
    }

    #[test]
    fn case_04() {
        let vec = vec![Some(1), Some(2)];
        let root = from_vec_to_bt(&vec);
        let result1 = Solution::zigzag_level_order(root);
        let expected: Vec<Vec<i32>> = vec![vec![1], vec![2]];
        assert_eq!(result1, expected);
    }

    #[test]
    fn case_05() {
        let vec = vec![Some(1), Some(2), Some(3), Some(4), None, None, Some(5)];
        let root = from_vec_to_bt(&vec);
        let result1 = Solution::zigzag_level_order(root);
        let expected: Vec<Vec<i32>> = vec![vec![1], vec![3, 2], vec![4, 5]];
        assert_eq!(result1, expected);
    }

    #[test]
    fn case_06() {
        let vec = vec![
            Some(0),
            Some(1),
            Some(2),
            Some(3),
            Some(4),
            Some(5),
            Some(6),
            Some(7),
            Some(8),
            Some(9),
            Some(10),
            Some(11),
            Some(12),
            Some(13),
            Some(14),
        ];
        let root = from_vec_to_bt(&vec);
        let result1 = Solution::zigzag_level_order(root);
        let expected: Vec<Vec<i32>> = vec![
            vec![0],
            vec![2, 1],
            vec![3, 4, 5, 6],
            vec![14, 13, 12, 11, 10, 9, 8, 7],
        ];
        assert_eq!(result1, expected);
    }
}
