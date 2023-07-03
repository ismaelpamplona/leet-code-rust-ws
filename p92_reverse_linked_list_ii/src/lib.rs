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
    pub fn reverse_between_it(
        head: Option<Box<ListNode>>,
        left: i32,
        right: i32,
    ) -> Option<Box<ListNode>> {
        let mut dummy = Some(Box::new(ListNode {
            val: -1,
            next: head,
        }));
        let mut before = &mut dummy;
        for _ in 1..left {
            before = &mut before.as_mut()?.next;
        }
        let mut prev = before.as_mut()?.next.take();
        let mut cur: Option<Box<ListNode>> = prev.as_mut()?.next.take();
        for _ in left..right {
            let next_node = cur.as_mut()?.next.take();
            cur.as_mut()?.next = prev;
            prev = cur;
            cur = next_node;
        }

        let mut tail = &mut prev;
        for _ in left..right {
            tail = &mut tail.as_mut()?.next;
        }
        tail.as_mut()?.next = cur;
        before.as_mut()?.next = prev;
        println!("{:?}", before);

        dummy.unwrap().next
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
        let head = from_vec_to_list_it(vec![1, 2, 3, 4, 5]);
        let output: Vec<i32> = vec![1, 4, 3, 2, 5];
        let result1 = Solution::reverse_between_it(head.clone(), 2, 4);
        assert_eq!(from_list_to_vec(result1), output);
    }

    // #[test]
    // fn case_02() {
    //     let head = from_vec_to_list_it(vec![5]);
    //     let output: Vec<i32> = vec![5];
    //     let result1 = Solution::reverse_between_it(head.clone(), 1, 1);
    //     assert_eq!(from_list_to_vec(result1), output);
    // }

    // #[test]
    // fn case_03() {
    //     let head = from_vec_to_list_it(vec![1, 2, 3, 4, 5, 6, 7]);
    //     let output: Vec<i32> = vec![1, 2, 5, 4, 3, 6, 7];
    //     let result1 = Solution::reverse_between_it(head.clone(), 3, 5);
    //     assert_eq!(from_list_to_vec(result1), output);
    // }
}

pub fn from_vec_to_list_it(vec: Vec<i32>) -> Option<Box<ListNode>> {
    if vec.is_empty() {
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
    if n == vec.len() as i32 {
        return None;
    }
    let mut head = ListNode::new(vec[n as usize]);
    head.next = from_vec_to_list_rec(vec, n + 1);
    Some(Box::new(head))
}

pub fn from_list_to_vec(head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];
    let mut cur = head;
    while let Some(node) = cur {
        result.push(node.val);
        cur = node.next
    }
    result
}
