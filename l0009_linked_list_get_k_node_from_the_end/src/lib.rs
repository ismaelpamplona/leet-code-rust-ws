use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct ListNode {
    val: i32,
    next: Option<Rc<RefCell<ListNode>>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

struct Solution;
impl Solution {
    pub fn get_k_node_from_the_end(
        head: Option<Rc<RefCell<ListNode>>>,
        k: i32,
    ) -> Option<Rc<RefCell<ListNode>>> {
        let mut slow = head.as_ref().map(|node| Rc::clone(node));
        let mut fast = head.as_ref().map(|node| Rc::clone(node));
        let mut count = 0;
        while fast.is_some() && count < k {
            fast = fast.as_ref().and_then(|node| node.borrow().next.clone());
            count += 1;
        }
        while slow.is_some() && fast.is_some() {
            slow = slow.as_ref().and_then(|node| node.borrow().next.clone());
            fast = fast.as_ref().and_then(|node| node.borrow().next.clone());
        }
        slow
    }

    pub fn compare(
        node1: Option<Rc<RefCell<ListNode>>>,
        node2: Option<Rc<RefCell<ListNode>>>,
    ) -> bool {
        let mut one = node1.as_ref().map(|node| Rc::clone(node));
        let mut two = node2.as_ref().map(|node| Rc::clone(node));
        let mut ans = true;
        while one.is_some() && two.is_some() {
            if let (Some(one_value), Some(two_value)) = (&one, &two) {
                if one_value.borrow().val != two_value.borrow().val {
                    ans = false;
                }
            }
            one = one.as_ref().and_then(|node| node.borrow().next.clone());
            two = two.as_ref().and_then(|node| node.borrow().next.clone());
        }

        ans
    }
}

mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let head = Some(Rc::new(RefCell::new(ListNode::new(1))));
        let second = Some(Rc::new(RefCell::new(ListNode::new(2))));
        let third = Some(Rc::new(RefCell::new(ListNode::new(3))));
        let fourth = Some(Rc::new(RefCell::new(ListNode::new(4))));
        let fifth = Some(Rc::new(RefCell::new(ListNode::new(5))));

        head.as_ref().unwrap().borrow_mut().next = second.clone();
        second.as_ref().unwrap().borrow_mut().next = third.clone();
        third.as_ref().unwrap().borrow_mut().next = fourth.clone();
        fourth.as_ref().unwrap().borrow_mut().next = fifth.clone();

        let result1 = Solution::get_k_node_from_the_end(head.clone(), 2);
        assert!(Solution::compare(result1, fourth.clone()));
    }
}
