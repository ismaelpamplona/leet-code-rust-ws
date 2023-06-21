#[derive(Debug)]
pub struct Node<T> {
    val: T,
    next: Option<Box<Node<T>>>,
}

fn add_node<T: std::default::Default>(
    prev_node: &mut Option<Box<Node<T>>>,
    node_to_add: &mut Box<Node<T>>,
) {
    node_to_add.next = prev_node.as_mut().and_then(|node| node.next.take());
    let node_to_add_imut = std::mem::replace(
        node_to_add,
        Box::new(Node {
            val: Default::default(),
            next: None,
        }),
    );
    *prev_node = Some(node_to_add_imut);
}

pub fn get_sum<T>(head: Option<Box<Node<T>>>) -> T
where
    T: std::ops::Add<Output = T> + Default + Copy,
{
    let mut ans = T::default();
    let mut current = head;
    while let Some(node) = current {
        ans = ans + node.val;
        current = node.next;
    }
    ans
}

pub fn get_sum_rec<T>(head: Option<Box<Node<T>>>) -> T
where
    T: std::ops::Add<Output = T> + Default + Copy,
{
    if head.is_none() {
        return T::default();
    }
    let node = head.unwrap();
    node.val + get_sum_rec(node.next)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_get_sum() {
        let node1 = Box::new(Node {
            val: 2,
            next: Some(Box::new(Node { val: 1, next: None })),
        });
        let node2 = Box::new(Node {
            val: 4,
            next: Some(Box::new(Node { val: 3, next: None })),
        });

        let sum1 = get_sum(Some(node1));
        assert_eq!(sum1, 3);

        let sum2 = get_sum_rec(Some(node2));
        assert_eq!(sum2, 7);
    }

    #[test]
    fn case_add_node() {
        let node1 = &mut Some(Box::new(Node {
            val: 2,
            next: Some(Box::new(Node {
                val: 1,
                next: Some(Box::new(Node { val: 0, next: None })),
            })),
        }));
        println!("{:?}", node1);
        let node2 = &mut Box::new(Node { val: 4, next: None });
        add_node(node1, node2);
        println!("{:?}", node1);
    }
}
