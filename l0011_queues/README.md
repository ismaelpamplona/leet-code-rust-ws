# Queues

- Follows **FIFO** (first in first out) pattern.
- Elements are added and removed from opposite sides.
- Example: if multiple people are trying to use a printer at the same time.

## Time Complexity
- Dependes on the implementation. 
- You can use a dynamic array, but operations on the front of the array (adding or removal) are $O(n)$, where $n$ is the size of the array.
- If you want these operations to be $O(1)$, you'll need a more sophisticated implementation. 
- One way to implement an efficient queue is by using a **doubly linked list**. If you have the pointer to a node, you can add or delete at that location in $O(1)$. A doubly linked list that maintains pointers to the head and tail (both ends, usually with sentinel nodes) can implement an efficient queue.
- There is also a data structure called a **deque**. In a deque, you can add or delete elements from both ends. A normal queue designates adding to one end and deleting to another end.
- The most common use of a queue is to implement an algorithm called **breadth-first search (BFS)**.


## Stacks and Recursion

- Recursion is actually done using a stack. 
- Function calls are pushed on a stack. 
- The call at the top of the stack at any given moment is the "active" call. 
- On a return statement or the end of the function being reached, the current call is popped off the stack.

## Interface guide

### Rust
```Rust
use std::collections::VecDeque;

// Declaration: Rust uses VecDeque from std::collections
let mut queue: VecDeque<i32> = VecDeque::new();

// Enqueueing/adding elements:
queue.push_back(1);
queue.push_back(2);
queue.push_back(3);

// Dequeing/removing elements:
queue.pop_front();

// Check if empty:
let is_empty = queue.is_empty(); // false

// Check element at front of queue (next element to be removed):
let front = *queue.front().unwrap(); // 2

// Get size:
let size = queue.len(); // 2
```

### C++
```C++
// Declaration: C++ supports multiple implementations, but we will be using
// std::queue. Specify the data type
queue<int> queue;

// Enqueueing/adding elements:
queue.push(1);
queue.push(2);
queue.push(3);

// Dequeing/removing elements:
queue.pop();

// Check if empty
queue.empty(); // false

// Check element at front of queue (next element to be removed)
queue.front(); // 2

// Get size
queue.size(); // 2
```

### Typescript
```Typescript
// Typescript doesn't have any built-in efficient queue
// We'll just have to use a normal array
let queue: number[] = [];

// If you want to initialize it with some initial values:
let queue: number[]  = [1, 2, 3];

// Enqueueing/adding elements:
queue.push(4);
queue.push(5);

// Dequeuing/removing elements:
queue.shift(); // 1
queue.shift(); // 2

// Check element at front of queue (next element to be removed)
queue[0]; // 3

// Get size
queue.length; // 3
```

### Python
```Python
# Declaration: we will use deque from the collections module
import collections
queue = collections.deque()

# If you want to initialize it with some initial values:
queue = collections.deque([1, 2, 3])

# Enqueueing/adding elements:
queue.append(4)
queue.append(5)

# Dequeuing/removing elements:
queue.popleft() # 1
queue.popleft() # 2

# Check element at front of queue (next element to be removed)
queue[0] # 3

# Get size
len(queue) # 3
```
