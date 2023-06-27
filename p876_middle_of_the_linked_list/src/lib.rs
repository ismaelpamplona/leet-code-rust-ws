use std::collections::LinkedList;

// Definition for singly-linked list.
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
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if let None = head {
            return None;
        }

        let mut q = vec![];
        if let Some(mut node) = head {
            let mut condition = true;
            while condition {
                q.push(node.clone());
                if let Some(next) = node.next {
                    node = next;
                } else {
                    condition = false;
                }
            }
        }

        Some(q[q.len() / 2].clone())
    }

    pub fn middle_node_2(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut mid = head.as_ref();
        let mut end = head.as_ref();

        while end.is_some() && end.as_ref().unwrap().next.is_some() {
            mid = mid.unwrap().next.as_ref();
            end = end.unwrap().next.as_ref().unwrap().next.as_ref();
        }

        mid.cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let head_1 = from_vec_to_list(vec![1, 2, 3, 4, 5], 0);
        let head_2 = from_vec_to_list(vec![1, 2, 3, 4, 5], 0);
        let output = vec![3, 4, 5];
        let result_1 = Solution::middle_node(head_1);
        let result_2 = Solution::middle_node_2(head_2);
        assert_eq!(from_list_to_vec(result_1), output);
        assert_eq!(from_list_to_vec(result_2), output);
    }

    #[test]
    fn case_02() {
        let head_1 = from_vec_to_list(vec![1, 2, 3, 4, 5, 6], 0);
        let head_2 = from_vec_to_list(vec![1, 2, 3, 4, 5, 6], 0);
        let output = vec![4, 5, 6];
        let result_1 = Solution::middle_node(head_1);
        let result_2 = Solution::middle_node_2(head_2);
        assert_eq!(from_list_to_vec(result_1), output);
        assert_eq!(from_list_to_vec(result_2), output);
    }

    #[test]
    fn case_from_vec_to_list() {
        let head = vec![1, 2, 3, 4];
        let result = from_vec_to_list(head, 0);
        let output = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode { val: 4, next: None })),
                })),
            })),
        }));

        assert_eq!(result, output);
    }

    #[test]
    fn case_from_list_to_vec() {
        let head = from_vec_to_list(vec![1, 2, 3, 4, 5, 6], 0);
        let output = vec![1, 2, 3, 4, 5, 6];
        let result = from_list_to_vec(head);
        assert_eq!(result, output);
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

// pub fn from_list_to_vec(linked_list: Option<Box<ListNode>>) -> Vec<i32> {
//     if let None = linked_list {
//         return vec![];
//     }
//     let mut linked_list_vector: Vec<i32> = vec![];

//     if let Some(mut node) = linked_list {
//         let mut condition = true;
//         while condition {
//             linked_list_vector.push(node.val);

//             if let Some(next) = node.next {
//                 node = next;
//             } else {
//                 condition = false;
//             }
//         }
//     }

//     linked_list_vector
// }

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
