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

#[derive(Debug, PartialEq, Eq)]
pub struct GraphNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<GraphNode>>>,
    pub right: Option<Rc<RefCell<GraphNode>>>,
    pub parent: Option<Rc<RefCell<GraphNode>>>,
}

impl GraphNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        GraphNode {
            val,
            left: None,
            right: None,
            parent: None,
        }
    }
}

type Tree = Option<Rc<RefCell<TreeNode>>>;
type Graph = Option<Rc<RefCell<GraphNode>>>;

fn from_vec_to_bt(vec: &[Option<i32>]) -> Tree {
    let tree = Rc::new(RefCell::new(TreeNode::new((vec.get(0))?.unwrap())));
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

fn from_bt_to_vec(root: &Tree) -> Vec<Option<i32>> {
    let mut result = vec![];
    let mut q = VecDeque::new();
    if let Some(node) = root {
        result.push(Some(node.borrow().val));
        q.push_back(Rc::clone(node));
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

fn from_bt_to_graph(root: Tree) -> Graph {
    let root_rc = Rc::clone(&root?);
    let graph = Rc::new(RefCell::new(GraphNode::new(root_rc.borrow().val)));
    let mut stack: Vec<(Rc<RefCell<GraphNode>>, Rc<RefCell<TreeNode>>, Graph)> =
        vec![(Rc::clone(&graph), Rc::clone(&root_rc), None)];
    while let Some((graph_rc, tree_rc, self_parent)) = stack.pop() {
        let GraphNode {
            left,
            right,
            parent,
            .. // explicitly ignore missing fields
        } = &mut *graph_rc.borrow_mut();
        if let Some(node) = self_parent {
            *parent = Some(Rc::clone(&node));
        }
        if let Some(nd) = &tree_rc.borrow().left {
            let new_graph = Rc::new(RefCell::new(GraphNode::new(nd.borrow().val)));
            *left = Some(Rc::clone(&new_graph));
            stack.push((new_graph, Rc::clone(nd), Some(Rc::clone(&graph_rc))));
        }
        if let Some(nd) = &tree_rc.borrow().right {
            let new_graph = Rc::new(RefCell::new(GraphNode::new(nd.borrow().val)));
            *right = Some(Rc::clone(&new_graph));
            stack.push((new_graph, Rc::clone(nd), Some(Rc::clone(&graph_rc))));
        }
    }
    Some(graph)
}

fn get_target(node: Graph, target: i32) -> Graph {
    let node_rc = node?;
    let mut seen = HashSet::from([node_rc.as_ref().borrow().val]);
    let mut queue = VecDeque::from([Rc::clone(&node_rc)]);

    while let Some(nd) = queue.pop_front() {
        let GraphNode {
            val,
            left,
            right,
            parent,
        } = &*nd.borrow();
        if *val == target {
            return Some(Rc::clone(&nd));
        }
        for leaf in [left, right, parent] {
            if let Some(neighbour) = leaf {
                if seen.insert(neighbour.as_ref().borrow().val) {
                    queue.push_back(Rc::clone(neighbour));
                }
            }
        }
    }

    None
}

struct Solution;
impl Solution {
    pub fn distance_k_map(
        root: Option<Rc<RefCell<TreeNode>>>,
        target: Option<Rc<RefCell<TreeNode>>>,
        k: i32,
    ) -> Vec<i32> {
        let mut stack = vec![];
        if let Some(node) = root {
            stack.push(Rc::clone(&node));
        }
        let mut graph = HashMap::new();
        while let Some(node) = stack.pop() {
            let node = node.borrow();
            for i in 0..=1 {
                if let Some(leaf) = &node[i] {
                    stack.push(Rc::clone(leaf));
                    graph
                        .entry(node.val)
                        .or_insert(vec![])
                        .push(leaf.borrow().val);
                    graph
                        .entry(leaf.borrow().val)
                        .or_insert(vec![])
                        .push(node.val);
                }
            }
        }
        let target = target.unwrap().borrow().val;
        let mut q = VecDeque::from([target]);
        let mut distance = 0;
        let mut seen = HashSet::from([target]);
        while !q.is_empty() && distance < k {
            let row_num_els = q.len();
            for _ in 0..row_num_els {
                let node = q.pop_front().unwrap();
                if let Some(vec) = graph.get(&node) {
                    for neighbour in vec {
                        if !seen.contains(neighbour) {
                            seen.insert(*neighbour);
                            q.push_back(*neighbour);
                        }
                    }
                }
            }
            distance += 1;
        }
        let mut result = vec![];
        for val in q {
            result.push(val);
        }
        result
    }

    pub fn distance_k_graph(
        root: Option<Rc<RefCell<TreeNode>>>,
        target: Option<Rc<RefCell<TreeNode>>>,
        k: i32,
    ) -> Vec<i32> {
        let graph = from_bt_to_graph(root);
        let target_node = get_target(graph, target.unwrap().borrow().val);
        if target_node.is_none() {
            return vec![];
        }
        let mut seen = HashSet::new();
        let mut q = VecDeque::new();
        if let Some(target) = target_node {
            seen.insert(target.borrow().val);
            q.push_back(Rc::clone(&target));
        }
        let mut distance = 0;
        while !q.is_empty() && distance < k {
            let row_num_els = q.len();
            for _ in 0..row_num_els {
                let node = q.pop_front().unwrap();
                let GraphNode {
                    left,
                    right,
                    parent,
                    ..
                } = &*node.borrow();
                for leaf in [left, right, parent] {
                    if let Some(neighbour) = leaf {
                        let val = neighbour.borrow().val;
                        if !seen.contains(&val) {
                            seen.insert(val);
                            q.push_back(Rc::clone(neighbour));
                        }
                    }
                }
            }
            distance += 1;
        }
        let mut result = vec![];
        for rc in q {
            result.push(rc.borrow().val);
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
        let bt_from_vec1: Tree = from_vec_to_bt(&vec1);
        let vec2: Vec<Option<i32>> = vec![Some(0), Some(1), None, Some(2), None, Some(3)];
        let bt_from_vec2: Tree = from_vec_to_bt(&vec2);
        let vec1_from_bt = from_bt_to_vec(&bt_from_vec1);
        let vec2_from_bt = from_bt_to_vec(&bt_from_vec2);
        assert_eq!(vec1, vec1_from_bt);
        assert_eq!(vec2, vec2_from_bt);
    }

    #[test]
    fn case_01() {
        let vec = vec![
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
        let root = from_vec_to_bt(&vec);
        let target = from_vec_to_bt(&vec![Some(5)]);
        let result1 = Solution::distance_k_map(root.clone(), target.clone(), 2);
        let result2 = Solution::distance_k_graph(root, target, 2);
        assert_eq!(result1, vec![1, 7, 4]);
        assert_eq!(result2, vec![7, 4, 1]);
    }

    #[test]
    fn case_02() {
        let vec = vec![Some(1)];
        let root = from_vec_to_bt(&vec);
        let target = from_vec_to_bt(&vec![Some(1)]);
        let result1 = Solution::distance_k_map(root.clone(), target.clone(), 3);
        let result2 = Solution::distance_k_graph(root, target, 3);
        assert_eq!(result1, vec![]);
        assert_eq!(result2, vec![]);
    }
}
