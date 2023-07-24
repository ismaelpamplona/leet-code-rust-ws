use std::cell::RefCell;
use std::collections::{HashMap, HashSet, VecDeque};
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
        for j in 0..2 {
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
    let mut result = vec![];
    let mut q = VecDeque::new();
    if let Some(node) = root {
        result.push(Some(node.borrow().val));
        q.push_back(Rc::clone(node));
    }

    while let Some(node) = q.pop_front() {
        let node = node.borrow();
        for i in 0..2 {
            if let Some(leaf) = &node[i] {
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
    pub fn lowest_common_ancestor_rec(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if let (Some(rn), Some(pn), Some(qn)) = (&root, &p, &q) {
            if rn.borrow().val == pn.borrow().val || rn.borrow().val == qn.borrow().val {
                return root;
            }

            let left = Self::lowest_common_ancestor_rec(
                rn.borrow().left.as_ref().map(Rc::clone),
                p.as_ref().map(Rc::clone),
                q.as_ref().map(Rc::clone),
            );

            let right = Self::lowest_common_ancestor_rec(
                rn.borrow().right.as_ref().map(Rc::clone),
                p.as_ref().map(Rc::clone),
                q.as_ref().map(Rc::clone),
            );

            if left.is_some() && right.is_some() {
                return root;
            }

            if left.is_some() {
                return left;
            }

            return right;
        }
        None
    }

    pub fn lowest_common_ancestor_it(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut stack = Vec::new();
        let mut parent_map: HashMap<i32, Rc<RefCell<TreeNode>>> = HashMap::new();

        if let Some(root_node) = root {
            stack.push(Rc::clone(&root_node));
        }

        while !parent_map.contains_key(&p.as_ref().unwrap().borrow().val)
            || !parent_map.contains_key(&q.as_ref().unwrap().borrow().val)
        {
            if let Some(node) = stack.pop() {
                let node_val = node.borrow().val;

                if let Some(left) = &node.borrow().left {
                    parent_map.insert(left.borrow().val, Rc::clone(&node));
                    stack.push(Rc::clone(left));
                }

                if let Some(right) = &node.borrow().right {
                    parent_map.insert(right.borrow().val, Rc::clone(&node));
                    stack.push(Rc::clone(right));
                }
            } else {
                break;
            }
        }

        let mut set_ancestors = HashSet::new();
        let mut p_ref = p.as_ref();
        while let Some(p_node) = p_ref {
            set_ancestors.insert(p_node.borrow().val);
            p_ref = parent_map.get(&p_node.borrow().val);
        }

        let mut q_ref = q.as_ref();
        while let Some(q_node) = q_ref {
            if set_ancestors.contains(&q_node.borrow().val) {
                return Some(Rc::clone(q_node));
            }
            q_ref = parent_map.get(&q_node.borrow().val);
        }

        None
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
        let root_vec = vec![
            Some(3),
            Some(5),
            Some(1),
            Some(6),
            Some(2),
            Some(0),
            Some(8),
            None,
            None,
            Some(7),
            Some(4),
        ];
        let p_vec = vec![Some(5)];
        let q_vec = vec![Some(1)];
        let root = from_vec_to_bt(&root_vec);
        let p = from_vec_to_bt(&p_vec);
        let q = from_vec_to_bt(&q_vec);
        let result1 = Solution::lowest_common_ancestor_rec(root.clone(), p.clone(), q.clone());
        let result2 = Solution::lowest_common_ancestor_it(root.clone(), p.clone(), q.clone());
        assert_eq!(result1.unwrap().borrow().val, 3);
        assert_eq!(result2.unwrap().borrow().val, 3);
    }

    #[test]
    fn case_02() {
        let root_vec = vec![
            Some(3),
            Some(5),
            Some(1),
            Some(6),
            Some(2),
            Some(0),
            Some(8),
            None,
            None,
            Some(7),
            Some(4),
        ];
        let p_vec = vec![Some(5)];
        let q_vec = vec![Some(4)];
        let root = from_vec_to_bt(&root_vec);
        let p = from_vec_to_bt(&p_vec);
        let q = from_vec_to_bt(&q_vec);
        let result1 = Solution::lowest_common_ancestor_rec(root.clone(), p.clone(), q.clone());
        let result2 = Solution::lowest_common_ancestor_it(root.clone(), p.clone(), q.clone());
        assert_eq!(result1.unwrap().borrow().val, 5);
        assert_eq!(result2.unwrap().borrow().val, 5);
    }

    #[test]
    fn case_03() {
        let root_vec = vec![Some(1), Some(2)];
        let p_vec = vec![Some(1)];
        let q_vec = vec![Some(2)];
        let root = from_vec_to_bt(&root_vec);
        let p = from_vec_to_bt(&p_vec);
        let q = from_vec_to_bt(&q_vec);
        let result1 = Solution::lowest_common_ancestor_rec(root.clone(), p.clone(), q.clone());
        let result2 = Solution::lowest_common_ancestor_it(root.clone(), p.clone(), q.clone());
        assert_eq!(result1.unwrap().borrow().val, 1);
        assert_eq!(result2.unwrap().borrow().val, 1);
    }
}
