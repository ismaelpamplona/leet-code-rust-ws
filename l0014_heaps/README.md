# Heaps

- A heap is a data structure that is an implementation of the [priority queue](https://en.wikipedia.org/wiki/Priority_queue).

- A heap is a container that stores elements, and supports the following operations:
    - Add an element in $O(log n)$
    - Remove the minimum element in $O(log n)$
    - Find the minimum element in $O(1)$

> A heap can also find the max elements instead of the min elements. If a heap is configured to find/remove the min element, it's called a min heap. If it's configured to find/remove the max element, it's called a max heap.

## How is a heap implemented?

### Binary Heap

- Is the most popular 
- A binary heap implements a binary tree, but with only an array.
- The idea is that each element in the array is a node in the tree. 
- The smallest element in the tree is the root, and the following property is maintained at every node: if `A` is the parent of `B`, then `A.val <= B.val`. 
- Another constraint is that the tree must be a [**complete tree**](https://en.wikipedia.org/wiki/Binary_tree#complete).
    - The parent-child relationships are done using math with the indices. 
        - `[0]`: root
        - `[1]`: root's left child
        - `[2]`: root's right child
        - `[3]`: 1's left child ($2 * 1 + 1 = 3$)
        - `[4]`: 1's right child ($2 * 1 + 2 = 4$)
        - `[5]`: 2's left child ($2 * 2 + 1 = 5$)
        - `[6]`: 2's right child ($2 * 2 + 2 = 6$)
        - . . . 
        - `[m]`: `i`'s left child ($2 * i + 1 = m$)
        - `[n]`: `i`'s right child ($2 * i + 2 = n$)
    - A complete binary tree is a binary tree whose all levels except the last level are completely filled and all the leaves in the last level are all to the left side. 

        <img src="https://media.geeksforgeeks.org/wp-content/uploads/CompleteBinaryTree.png" style="max-height: 200px"/>

- In many problems, using a heap can improve an algorithm's time complexity from $O(n⋅log n)$, which is a massive improvement (for n = 1,000,000, this is 50,000 times faster). A heap is a great option whenever you need to find the maximum or minimum of something repeatedly.

### Interface guide

#### Rust
```rust
use std::collections::BinaryHeap;

// Declaration: Create a min heap using BinaryHeap
let mut heap = BinaryHeap::new();

// Add to heap
heap.push(1);
heap.push(2);
heap.push(3);

// Check maximum element
if let Some(max) = heap.peek() {
    println!("Maximum element: {}", max); // 3
}

// Pop maximum element
if let Some(max) = heap.pop() {
    println!("Popped maximum element: {}", max); // 3
}

// Get size
let size = heap.len();
println!("Heap size: {}", size); // 2

// Bonus: convert a vector to a heap in linear time
let mut nums = vec![43, 2, 13, 634, 120];
let mut heap_from_vec = BinaryHeap::from(nums);

// Now, you can use push and pop on heap_from_vec
if let Some(max) = heap_from_vec.pop() {
    println!("Popped maximum element from heap_from_vec: {}", max); // 634
}
```
#### Python
```python
# In Python, we will use the heapq module
# Note: heapq only implements min heaps
from heapq import *

# Declaration: heapq does not give you a heap data structure.
# You just use a normal list, and heapq provides you with
# methods that can be used on this list to perform heap operations
heap = []

# Add to heap
heappush(heap, 1)
heappush(heap, 2)
heappush(heap, 3)

# Check minimum element
heap[0] # 1

# Pop minimum element
heappop(heap) # 1

# Get size
len(heap) # 2

# Bonus: convert a list to a heap in linear time
nums = [43, 2, 13, 634, 120]
heapify(nums)

# Now, you can use heappush and heappop on nums
# and nums[0] will always be the minimum element
```


