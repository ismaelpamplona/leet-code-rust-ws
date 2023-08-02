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
    let mut result = Vec::new();
    let mut q = VecDeque::new();

    if let Some(root) = root {
        q.push_back(Rc::clone(root));
        result.push(Some(root.borrow().val));
    }

    while let Some(node) = q.pop_front() {
        let node = node.borrow();

        for i in 0..=1 {
            if let Some(node) = &node[i] {
                q.push_back(Rc::clone(node));
                result.push(Some(node.borrow().val));
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
    pub fn dfs_rec(root: &Option<Rc<RefCell<TreeNode>>>) {
        if let Some(node) = root {
            let borrowed = node.borrow();
            println!("{}", borrowed.val);
            Self::dfs_rec(&borrowed.left); // 1
            Self::dfs_rec(&borrowed.right); // 2
        }
    }

    pub fn dfs_it(root: &Option<Rc<RefCell<TreeNode>>>) {
        let mut stack = vec![];
        if let Some(node) = root {
            stack.push(Rc::clone(node));
        }
        while let Some(node) = stack.pop() {
            let node = node.borrow();
            println!("{}", node.val);
            for i in (0..=1).rev() {
                if let Some(leaf) = &node[i] {
                    stack.push(Rc::clone(leaf));
                }
            }
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

    pub fn dfs_sum(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root {
            let borrowed = node.borrow();
            let left = Self::dfs_sum(&borrowed.left); // 1
            let right = Self::dfs_sum(&borrowed.right); // 2
            return node.borrow().val + left + right;
        }
        0
    }

    pub fn dfs_max_depth(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root {
            let borrowed = node.borrow();
            let left = Self::dfs_max_depth(&borrowed.left); // 1
            let right = Self::dfs_max_depth(&borrowed.right); // 2
            return left.max(right) + 1;
        }
        0
    }

    pub fn dfs_lca(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if let (Some(rn), Some(pn), Some(qn)) = (&root, &p, &q) {
            if rn.borrow().val == pn.borrow().val || rn.borrow().val == qn.borrow().val {
                return root;
            }

            let left = Self::dfs_lca(
                rn.borrow().left.as_ref().map(Rc::clone),
                Some(Rc::clone(pn)),
                Some(Rc::clone(qn)),
            );
            let right = Self::dfs_lca(
                rn.borrow().right.as_ref().map(Rc::clone),
                Some(Rc::clone(pn)),
                Some(Rc::clone(qn)),
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

    pub fn bfs_it(root: &Option<Rc<RefCell<TreeNode>>>) {
        let mut q = VecDeque::new();
        if let Some(node) = root {
            q.push_back(Rc::clone(&node));
        }

        while let Some(node) = q.pop_front() {
            let node = node.borrow();
            // do some logic here on the current node
            println!("{}", node.val);

            // put the next level onto the queue
            for j in 0..=1 {
                if let Some(leaf) = node[j].as_ref() {
                    q.push_back(Rc::clone(leaf));
                }
            }
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
        let node5 = Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: None,
            right: Some(node6),
        }));
        let node4 = Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: None,
            right: None,
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
        Solution::dfs_rec(&Some(root.clone()));
        Solution::dfs_it(&Some(root.clone()));
        Solution::preorder_dfs(&Some(root.clone()));
        Solution::inorder_dfs(&Some(root.clone()));
        Solution::postorder_dfs(&Some(root.clone()));
        println!("dfs_sum: {}", Solution::dfs_sum(&Some(root.clone())));
        println!(
            "dfs_max_depth: {}",
            Solution::dfs_max_depth(&Some(root.clone()))
        );
    }

    #[test]
    fn case_dfs_lca() {
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
        let result1 = Solution::dfs_lca(root.clone(), p.clone(), q.clone());
        assert_eq!(result1.unwrap().borrow().val, 3);
    }

    #[test]
    fn case_bfs() {
        let root_vec = vec![Some(0), Some(1), Some(2), Some(3), Some(4), Some(5)];
        let root = from_vec_to_bt(&root_vec);
        Solution::bfs_it(&root.clone());
    }

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
}
