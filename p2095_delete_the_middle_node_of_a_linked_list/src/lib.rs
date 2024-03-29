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
        let mut dummy = head;
        let mut fast = &(dummy.clone());
        let mut slow = &mut dummy;

        while fast.is_some() && fast.as_ref().unwrap().next.is_some() {
            slow = &mut slow.as_mut()?.next;
            fast = &fast.as_ref()?.next.as_ref()?.next;
        }

        *slow = (*slow).as_mut()?.next.take();

        dummy
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
        let head = from_vec_to_list_it(vec![1, 3, 4, 7, 1, 2, 6]);
        let output = vec![1, 3, 4, 1, 2, 6];
        let result = from_list_to_vec(Solution::delete_middle(head.clone()));
        assert_eq!(result, output);
    }

    #[test]
    fn case_02() {
        let head = from_vec_to_list_it(vec![1, 2, 3, 4]);
        let output = vec![1, 2, 4];
        let result = from_list_to_vec(Solution::delete_middle(head.clone()));
        assert_eq!(result, output);
    }

    #[test]
    fn case_03() {
        let head = from_vec_to_list_it(vec![2, 1]);
        let output = vec![2];
        let result = from_list_to_vec(Solution::delete_middle(head.clone()));
        assert_eq!(result, output);
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
