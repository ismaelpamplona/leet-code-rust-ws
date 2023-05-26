# p876_middle_of_the_linked_list
[https://leetcode.com/problems/middle-of-the-linked-list/](https://leetcode.com/problems/middle-of-the-linked-list/)

## Initial provided code
```Rust
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        
    }
}
```

So, at this point I know:
1. the parameter type;
2. the return type; and 
3. that LeetCode tests uses a vector as input.

## Auxiliary Functions

To solve this problem, I need to write two auxiliary functions:

### 1. Convert TreeNode to a vector:

```Rust
pub fn from_linked_list_to_vec(linked_list: Option<Box<ListNode>>) -> Vec<i32> {
    if let None = linked_list {
        return vec![];
    }
    let mut linked_list_vector: Vec<i32> = vec![];

    if let Some(mut node) = linked_list {
        let mut condition = true;
        while condition {
            linked_list_vector.push(node.val);

            if let Some(next) = node.next {
                node = next;
            } else {
                condition = false;
            }
        }
    }

    linked_list_vector
}
```

### 2. Covert a vector to LinkedList:
#### Finding the math relation of the node positions:

```Rust
pub fn from_vec_to_linked_list(vec: Vec<i32>, n: i32) -> Option<Box<ListNode>> {
    let mut linked_list = None;

    if vec.is_empty() {
        return None;
    } else if n < vec.len() as i32 {
        linked_list = Some(Box::new(ListNode::new(vec[n as usize])));
        linked_list.as_mut().unwrap().next = from_vec_to_linked_list(vec, n + 1);
    }
    linked_list
}
```

## First approach - Iterative

In this case, I iterated throw the linked list and copy each node to an `array`. After this, a returned the `array[n / 2]` to get the middle node.

In Rust, integer division (using the / operator) already performs "floor division" by default. This means that the result of integer division is always rounded down to the nearest integer.

```Rust
let result_1 = 5 / 2; // 2
let result_2 = 4 / 2; // 2
let result_3 = 7 / 3; // 2 
```

### Solution

```Rust
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
```

- n = number of nodes
- time complexity: O(n)
- space complexity: O(n)

## Second approach - Pointers

Copying an array, makes the space complexity increases a lot. As you can observe, in the first approach, the space complexity is O(n).

So, I started thinking how could I find the middle node without another data structure (like the array that I used before).

So I started trying to find some pattern:
- [<u>**1**</u>] => 1
- [1, <u>2</u>] => 2
- [1, <u>2</u>, 3] => 2
- [1, 2, <u>3</u>, 4] => 3
- [1, 2, <u>3</u>, 4, 5] => 3  
- [1, 2, 3, <u>4</u>, 5, 6] => 4
- [1, 2, 3, <u>4</u>, 5, 6, 7] => 4

As you can notice, as the size of my list grows by 2, my middle node moves up. So every time that <u>my current end node</u> and <u>the next of the current end node</u> are not null: `mid = mid.next` and `end = end.next.next`.

```Rust
  pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
      let mut mid = head.as_ref();
      let mut end = head.as_ref();

      while end.is_some() && end.as_ref().unwrap().next.is_some() {
          mid = mid.unwrap().next.as_ref();
          end = end.unwrap().next.as_ref().unwrap().next.as_ref();
      }

      mid.cloned()
  }
 ```


- n = number of nodes
- time complexity: O(n)
- space complexity: O(1)

