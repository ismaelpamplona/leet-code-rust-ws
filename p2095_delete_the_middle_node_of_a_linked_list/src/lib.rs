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
    pub fn delete_middle(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() || head.as_ref().unwrap().next.is_none() {
            return None;
        }

        let mut dummy = Some(Box::new(ListNode {
            val: -1,
            next: head,
        }));
        let mut slow = head.as_ref().unwrap().as_ref();
        let mut prev = head.as_ref().unwrap().as_ref();
        let mut fast = head.as_ref().unwrap().as_ref();

        while fast.next.is_some() {
            prev = slow;
            slow = slow.next.as_ref().unwrap().as_ref();
            fast = fast.next.as_ref().unwrap().next.as_ref().unwrap().as_ref();
        }

        let node = slow.next.take();
        prev.next = node;

        dummy.unwrap().next
        &
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_transformation() {
        let vec = vec![1, 2, 3, 4, 5];
        let head1 = from_vec_to_list_it(vec.clone());
        let head2 = from_vec_to_list_rec(vec.clone(), 0);
        let vec1 = from_list_to_vec(head1);
        let vec2 = from_list_to_vec(head2);
        assert_eq!(vec1, vec2);
    }
}

pub fn from_vec_to_list_it(vec: Vec<i32>) -> Option<Box<ListNode>> {
    if vec.len() == 0 {
        return None;
    }
    let mut head = Some(Box::new(ListNode::new(vec[0])));
    let mut cur = head.as_mut();
    for i in 1..vec.len() {
        let new_node = Some(Box::new(ListNode::new(vec[i])));
        let node = cur.take().unwrap();
        node.next = new_node;
        cur = node.next.as_mut();
    }
    head
}

pub fn from_vec_to_list_rec(vec: Vec<i32>, n: i32) -> Option<Box<ListNode>> {
    if n >= vec.len() as i32 {
        return None;
    }
    let mut head = ListNode::new(vec[n as usize]);
    head.next = from_vec_to_list_rec(vec, n + 1);
    Some(Box::new(head))
}

pub fn from_list_to_vec(head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut vec: Vec<i32> = vec![];
    let mut cur = head;
    while let Some(node) = cur {
        vec.push(node.val);
        cur = node.next;
    }
    return vec;
}
