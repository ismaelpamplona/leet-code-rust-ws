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
    pub fn get_k_node_from_the_end(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut slow = &head;
        let mut fast = &head;
        for i in 1..=k {
            fast = &fast.as_ref().unwrap().next;
        }
        while fast.is_some() {
            slow = &slow.as_ref().unwrap().next;
            fast = &fast.as_ref().unwrap().next;
        }
        slow.clone()
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

    #[test]
    fn case_01() {
        let vec = vec![1, 2, 3, 4, 5];
        let head = from_vec_to_list_it(vec);
        let result = Solution::get_k_node_from_the_end(head, 2);
        let output = vec![4, 5];
        assert_eq!(output, from_list_to_vec(result));
    }
}

pub fn from_vec_to_list_it(vec: Vec<i32>) -> Option<Box<ListNode>> {
    if vec.is_empty() {
        return None;
    }
    let mut head = Some(Box::new(ListNode::new(vec[0])));
    let mut cur = head.as_mut();
    // let mut ptr = &mut head;
    // for e in vec.iter().skip(1)
    for i in 1..vec.len() {
        cur.as_mut().unwrap().next = Some(Box::new(ListNode::new(vec[i])));
        cur = cur.unwrap().next.as_mut();
    }
    head
}

pub fn from_vec_to_list_rec(vec: Vec<i32>, n: usize) -> Option<Box<ListNode>> {
    if n >= vec.len() {
        return None;
    }
    let mut head = ListNode::new(vec[n]);
    head.next = from_vec_to_list_rec(vec, n + 1);
    Some(Box::new(head))
}

pub fn from_list_to_vec(head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut vec: Vec<i32> = vec![];
    let mut cur = head.as_ref();
    while let Some(node) = cur {
        vec.push(node.val);
        cur = node.next.as_ref();
    }
    vec
}
