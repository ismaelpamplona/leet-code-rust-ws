# Linked List

## Advantages

1. The **main advantage** of a linked list is that you can **add and remove elements at any position in $O(1)$**. The caveat is that you need to have a reference to a node at the position in which you want to perform the addition/removal, otherwise the operation is $O(n)$, because you will need to iterate starting from the head until you get to the desired position. However, this is still much better than a normal (dynamic) array, which requires $O(n)$ for adding and removing from an arbitrary position.
2. Linked lists have the advantage of **not having fixed sizes**. While dynamic arrays can be resized, under the hood they still are allocated a fixed size - it's just that when this size is exceeded, the array is resized, which is expensive. 

## Disadvantages

1. The **main disadvantage** of a linked list is that **there is no random access**. If you have a large linked list and want to access the 150,000th element, then there usually isn't a better way than to start at the head and iterate 150,000 times. So while an array has $O(1)$ indexing, a linked list could require $O(n)$ to access an element at a given position.
2. Linked lists have **more overhead than arrays** - every element needs to have extra storage for the pointers. If you are only storing small items like booleans or characters, then you may be more than doubling the space needed.

## Types of linked lists

### Singly linked list

- Each node only has a pointer to the next node. This means you can only move forward in the list when iterating.

```Rust
pub struct Node<T> {
    val: T,
    next: Option<Box<Node<T>>>,
}
```

### Doubly linked list

- Each node also contains a pointer to the previous node. This pointer is usually called prev, and it allows iteration in both directions.

```Rust
pub struct Node<T> {
    val: T,
    next: Option<Box<Node<T>>>,
    prev: Option<Box<Node<T>>>,
}
```

### Linked lists with sentinel nodes

- We call the start of a linked list the head and the end of a linked list the tail.
- A linked list with sentinel nodes is a variation of a linked list that uses additional special nodes, called sentinel nodes, at both poles of the list. These sentinel nodes act as placeholders and provide easier and more efficient list manipulation.