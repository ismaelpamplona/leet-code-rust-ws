use std::cell::RefCell;
use std::collections::HashSet;
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
    pub fn has_cycle_set(head: Option<Rc<RefCell<ListNode>>>) -> bool {
        let mut node_set = HashSet::new();
        let mut curr = head;

        while let Some(node_ref) = curr {
            let node = node_ref.borrow();

            if !node_set.insert(Rc::into_raw(Rc::clone(&node_ref)) as *const ()) {
                return true; // cycle detected
            }

            curr = node.next.clone();
        }

        false // no cycle detected
    }

    pub fn has_cycle_floyd(head: Option<Rc<RefCell<ListNode>>>) -> bool {
        let mut slow = head.as_ref().map(|node| Rc::clone(node));
        let mut fast = head.as_ref().map(|node| Rc::clone(node));

        while slow.is_some() && fast.is_some() {
            let next_slow = slow.as_ref().and_then(|node| node.borrow().next.clone());
            let next_fast = fast.as_ref().and_then(|node| node.borrow().next.clone());

            slow = next_slow;
            fast = next_fast;

            if slow.is_some()
                && fast.is_some()
                && Rc::ptr_eq(slow.as_ref().unwrap(), fast.as_ref().unwrap())
            {
                return true; // cycle detected
            }
        }

        false // no cycle detected
    }
}

mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let head = Some(Rc::new(RefCell::new(ListNode::new(3))));
        let second = Some(Rc::new(RefCell::new(ListNode::new(2))));
        let third = Some(Rc::new(RefCell::new(ListNode::new(0))));
        let fourth = Some(Rc::new(RefCell::new(ListNode::new(-4))));

        head.as_ref().unwrap().borrow_mut().next = second.clone();
        second.as_ref().unwrap().borrow_mut().next = third.clone();
        third.as_ref().unwrap().borrow_mut().next = fourth.clone();
        fourth.as_ref().unwrap().borrow_mut().next = second.clone();

        let result1 = Solution::has_cycle_set(head.clone());
        let result2 = Solution::has_cycle_floyd(head);

        assert_eq!(result1, true);
        assert_eq!(result2, true);
    }

    #[test]
    fn case_02() {
        let head = Some(Rc::new(RefCell::new(ListNode::new(2))));
        let second = Some(Rc::new(RefCell::new(ListNode::new(1))));

        head.as_ref().unwrap().borrow_mut().next = second.clone();
        second.as_ref().unwrap().borrow_mut().next = head.clone();

        let result1 = Solution::has_cycle_set(head.clone());
        let result2 = Solution::has_cycle_floyd(head);

        assert_eq!(result1, true);
        assert_eq!(result2, true);
    }

    #[test]
    fn case_03() {
        let head = Some(Rc::new(RefCell::new(ListNode::new(1))));

        let result1 = Solution::has_cycle_set(head.clone());
        let result2 = Solution::has_cycle_floyd(head);

        assert_eq!(result1, false);
        assert_eq!(result2, false);
    }
}
