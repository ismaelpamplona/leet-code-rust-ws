use std::collections::HashSet;

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
    pub fn delete_duplicates_straight_forward(
        mut head: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        if head.is_none() {
            return None;
        }
        let mut ptr1 = head.as_mut().unwrap();
        while let Some(ptr2) = ptr1.next.as_mut() {
            if ptr1.val == ptr2.val {
                ptr1.next = ptr2.next.take();
            } else {
                ptr1 = ptr1.next.as_mut().unwrap();
            }
        }
        head
    }

    pub fn delete_duplicates_set(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        head.as_ref()?;
        let mut set = HashSet::new();
        let mut dummy = Some(Box::new(ListNode::new(0)));
        let mut current = dummy.as_mut();

        let mut node = head;
        while let Some(inner) = node {
            if set.insert(inner.val) {
                if let Some(cur) = current {
                    cur.next = Some(Box::new(ListNode::new(inner.val)));
                    current = cur.next.as_mut();
                }
            }
            node = inner.next;
        }

        dummy.unwrap().next
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_from_vec_to_list_it() {
        let head = vec![1, 2, 3, 4];
        let result1 = from_vec_to_list_it(head.clone());
        let result2 = from_vec_to_list_rec(head, 0);
        assert_eq!(result1, result2);
    }

    #[test]
    fn case_from_list_to_vec() {
        let head = vec![1, 2, 3, 4];
        let list1 = from_vec_to_list_it(head.clone());
        let list2 = from_vec_to_list_rec(head.clone(), 0);
        let result1 = from_list_to_vec(list1);
        let result2 = from_list_to_vec(list2);
        assert_eq!(result1, head);
        assert_eq!(result2, head);
    }

    #[test]
    fn case_01() {
        let head = from_vec_to_list_it(vec![1, 1, 2]);
        let output = vec![1, 2];
        let unique_head1 = Solution::delete_duplicates_set(head.clone());
        let unique_head2 = Solution::delete_duplicates_straight_forward(head.clone());
        let result1 = from_list_to_vec(unique_head1);
        let result2 = from_list_to_vec(unique_head2);
        assert_eq!(result1, output);
        assert_eq!(result2, output);
    }

    #[test]
    fn case_02() {
        let head = from_vec_to_list_it(vec![1, 1, 2, 3, 3]);
        let output = vec![1, 2, 3];
        let unique_head1 = Solution::delete_duplicates_set(head.clone());
        let unique_head2 = Solution::delete_duplicates_straight_forward(head.clone());
        let result1 = from_list_to_vec(unique_head1);
        let result2 = from_list_to_vec(unique_head2);
        assert_eq!(result1, output);
        assert_eq!(result2, output);
    }

    #[test]
    fn case_03() {
        let head = None;
        let output = vec![];
        let unique_head1 = Solution::delete_duplicates_set(head.clone());
        let unique_head2 = Solution::delete_duplicates_straight_forward(head.clone());
        let result1 = from_list_to_vec(unique_head1);
        let result2 = from_list_to_vec(unique_head2);
        assert_eq!(result1, output);
        assert_eq!(result2, output);
    }
}

pub fn from_vec_to_list_it(vec: Vec<i32>) -> Option<Box<ListNode>> {
    if vec.is_empty() {
        return None;
    }
    let mut head: Option<Box<ListNode>> = Some(Box::new(ListNode::new(vec[0])));
    let mut cur = head.as_mut();

    for i in 1..vec.len() {
        let new_node = Some(Box::new(ListNode::new(vec[i])));
        if let Some(node) = cur {
            node.next = new_node;
            cur = node.next.as_mut();
        }
    }

    head
}

pub fn from_vec_to_list_rec(vec: Vec<i32>, n: i32) -> Option<Box<ListNode>> {
    if n >= vec.len() as i32 {
        return None;
    }
    let mut head = Some(Box::new(ListNode::new(vec[n as usize])));
    head.as_mut().unwrap().next = from_vec_to_list_rec(vec, n + 1);
    head
}

pub fn from_list_to_vec(head: Option<Box<ListNode>>) -> Vec<i32> {
    if head.is_none() {
        return vec![];
    }
    let mut vec: Vec<i32> = vec![];
    let mut h = head;
    while h.is_some() {
        let node = h.clone().unwrap();
        vec.push(node.val);
        h = node.next;
    }
    vec
}
