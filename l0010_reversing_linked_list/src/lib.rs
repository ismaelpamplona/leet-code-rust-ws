#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution;
impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev: Option<Box<ListNode>> = None;
        let mut cur = head;

        while cur.is_some() {
            if let Some(box_node) = cur.as_mut() {
                let next_node = box_node.next.take();
                box_node.next = prev.take();
                prev = cur;
                cur = next_node;
            }
        }
        prev
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let nums = vec![1, 2, 3, 4];
        let reversed = vec![4, 3, 2, 1];
        let head = from_vec_to_list(nums, 0);
        let result = Solution::reverse_list(head);
        let reversed_list = from_vec_to_list(reversed, 0);
        assert_eq!(result, reversed_list);
    }
}

pub fn from_vec_to_list(vec: Vec<i32>, n: i32) -> Option<Box<ListNode>> {
    let mut head = None;
    if vec.is_empty() {
        return None;
    } else if n < vec.len() as i32 {
        head = Some(Box::new(ListNode::new(vec[n as usize])));
        head.as_mut().unwrap().next = from_vec_to_list(vec, n + 1);
    }
    head
}

pub fn from_list_to_vec(linked_list: Option<Box<ListNode>>) -> Vec<i32> {
    if let None = linked_list {
        return vec![];
    }
    let mut linked_list_vector: Vec<i32> = vec![];
    let mut head = linked_list.clone();

    while head.is_some() {
        let node = head.clone().unwrap();
        linked_list_vector.push(node.val);
        head = node.next;
    }

    linked_list_vector
}
