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
    pub fn swap_pairs_it(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return None;
        }
        let mut dummy = Box::new(ListNode {
            val: -1,
            next: head,
        });
        let mut prev = &mut dummy;

        while let Some(mut fst) = prev.next.take() {
            if let Some(mut snd) = fst.next.take() {
                fst.next = snd.next.take();
                snd.next = Some(fst);
                prev.next = Some(snd);

                prev = prev.next.as_mut().unwrap().next.as_mut().unwrap();
            } else {
                prev.next = Some(fst);
                prev = prev.next.as_mut().unwrap();
            }
        }

        dummy.next
    }

    pub fn swap_pairs_rec(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut node = head;
        if let Some(ref mut fst) = node {
            if let Some(mut snd) = fst.next.take() {
                std::mem::swap(fst, &mut snd);
                snd.next = Self::swap_pairs_it(fst.next.take());
                fst.next = Some(snd);
            }
        }
        node
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
        let head1 = from_vec_to_list_it(vec![1, 2, 3, 4]);
        let head2 = from_vec_to_list_it(vec![1, 2, 3, 4]);
        let output = vec![2, 1, 4, 3];
        let result1 = Solution::swap_pairs_it(head1);
        let result2 = Solution::swap_pairs_rec(head2);
        assert_eq!(from_list_to_vec(result1), output);
        assert_eq!(from_list_to_vec(result2), output);
    }

    #[test]
    fn case_02() {
        let head1 = from_vec_to_list_it(vec![]);
        let head2 = from_vec_to_list_it(vec![]);
        let output = vec![];
        let result1 = Solution::swap_pairs_it(head1);
        let result2 = Solution::swap_pairs_rec(head2);
        assert_eq!(from_list_to_vec(result1), output);
        assert_eq!(from_list_to_vec(result2), output);
    }

    #[test]
    fn case_03() {
        let head1 = from_vec_to_list_it(vec![1]);
        let head2 = from_vec_to_list_it(vec![1]);
        let output = vec![1];
        let result1 = Solution::swap_pairs_it(head1);
        let result2 = Solution::swap_pairs_rec(head2);
        assert_eq!(from_list_to_vec(result1), output);
        assert_eq!(from_list_to_vec(result2), output);
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
        let node = cur.take().unwrap();
        node.next = new_node;
        cur = node.next.as_mut();
    }

    head
}

pub fn from_vec_to_list_rec(vec: Vec<i32>, n: i32) -> Option<Box<ListNode>> {
    if n == vec.len() as i32 {
        return None;
    }
    let mut head = ListNode::new(vec[n as usize]);
    head.next = from_vec_to_list_rec(vec, n + 1);

    Some(Box::new(head))
}

pub fn from_list_to_vec(head: Option<Box<ListNode>>) -> Vec<i32> {
    if head.is_none() {
        return vec![];
    }
    let mut vec: Vec<i32> = vec![];
    let mut cur = head;
    while let Some(node) = cur {
        vec.push(node.val);
        cur = node.next
    }
    vec
}
