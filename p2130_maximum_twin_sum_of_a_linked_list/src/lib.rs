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
    pub fn pair_sum(mut head: Option<Box<ListNode>>) -> i32 {
        let mut slow = &head;
        let mut fast = &head;
        while fast.is_some() {
            slow = &slow.as_ref().unwrap().next;
            fast = &fast.as_ref().unwrap().next.as_ref().unwrap().next;
        }

        let mut prev: Option<Box<ListNode>> = None;
        let mut cur = slow.clone();

        while let Some(mut node) = cur.take() {
            let next = node.next.take();
            node.next = prev.take();
            prev = Some(node);
            cur = next;
        }

        let mut back = prev;
        let mut start = head;
        let mut max_sum = 0;
        while let Some(mut second) = back.take() {
            let mut first = start.unwrap();
            max_sum = max_sum.max(first.val + second.val);
            start = first.next.take();
            back = second.next.take();
        }
        max_sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_transformation() {
        let vec = vec![1, 2, 3, 4, 5];
        let head1 = from_vec_to_list_it(&vec);
        let head2 = from_vec_to_list_rec(&vec, 0);
        let vec1 = from_list_to_vec(head1);
        let vec2 = from_list_to_vec(head2);
        assert_eq!(vec1, vec2);
    }

    #[test]
    fn case_01() {
        let head = from_vec_to_list_it(&vec![5, 4, 2, 1]);
        let result1 = Solution::pair_sum(head);
        assert_eq!(result1, 6);
    }

    #[test]
    fn case_02() {
        let head = from_vec_to_list_it(&vec![4, 2, 2, 3]);
        let result1 = Solution::pair_sum(head);
        assert_eq!(result1, 7);
    }

    #[test]
    fn case_03() {
        let head = from_vec_to_list_it(&vec![1, 100000]);
        let result1 = Solution::pair_sum(head);
        assert_eq!(result1, 100001);
    }
}

pub fn from_vec_to_list_it(vec: &Vec<i32>) -> Option<Box<ListNode>> {
    if vec.is_empty() {
        return None;
    }
    let mut head = Some(Box::new(ListNode::new(vec[0])));
    let mut cur = &mut head;
    for num in vec.iter().skip(1) {
        cur.as_mut().unwrap().next = Some(Box::new(ListNode::new(*num)));
        cur = &mut cur.as_mut().unwrap().next;
    }
    head
}

pub fn from_vec_to_list_rec(vec: &Vec<i32>, n: usize) -> Option<Box<ListNode>> {
    if n >= vec.len() {
        return None;
    }
    let mut head = Box::new(ListNode::new(vec[n]));
    head.next = from_vec_to_list_rec(vec, n + 1);
    Some(head)
}

pub fn from_list_to_vec(mut head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut vec: Vec<i32> = vec![];
    let mut cur = &mut head;
    while let Some(node) = cur {
        vec.push(node.val);
        cur = &mut node.next;
    }
    vec
}
