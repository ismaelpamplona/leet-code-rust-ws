#  Binary trees

## What is a node?

- A node is an abstract data type with two things. 
- First, a node stores data. This data can be whatever you want - an integer, a boolean, a hash map, your own custom objects, or all of the above. 
- Second, a node stores pointers to other nodes.

## What is a graph?

- A graph is any collection of nodes and their pointers to other nodes. 
- Linked lists and trees are both types of graphs. 
- As a topic, graphs are extremely broad. There is an entire field of study dedicated to graphs called [graph theory](https://en.wikipedia.org/wiki/Graph_theory).

    <img style="max-height: 400px" src="https://media.geeksforgeeks.org/wp-content/uploads/20200630111809/graph18.jpg"/>

## What is a tree?

- A tree is a type of graph.
- Trees and graphs are considered different topics when it comes to algorithm problems.
- In a tree, a node has pointers to its children.
- The root is the only node that has no parent. Note that in a tree, a node cannot have more than one parent.

    <img style="max-height: 400px" src="https://media.geeksforgeeks.org/wp-content/uploads/20191005131555/General-Tree.jpg"/>

- General trees are implemented all around us in real life. Examples:
    - File systems (root directory, and subfolders/files).
    - A comment thread on an app like Reddit or Twitter (original post/tweet, and the comments and replies).
    - A company's organization chart (CEO, and direct reports).

## Graphical representation

- Nodes of a graph are also called vertices. 
- The pointers that connect them are called edges. 
- In graphical representations, nodes/vertices are usually represented with circles and the edges are lines/arrows that connect the circles (we saw this in the linked lists chapter).
- The start of a linked list was called the head. 
- The start of a binary tree is called the root.

    <img style="max-height: 400px" src="https://media.geeksforgeeks.org/wp-content/cdn-uploads/undirectedgraph.png"/>

## Terminology

- **The root** node is the node at the "top" of the tree. Every node in the tree is accessible starting from the root node.
- If you have a node `A` with an edge to a node `B`, so `A -> B`
  - `A` the **parent** of node `B`, 
  - `B` a **child** of node `A`.
- If a node has no children, it is called a **leaf** node (both children are null). The leaf nodes are the leaves of the tree.
- The **depth** of a node is how far it is from the root node. The root has a depth of $0$. Every child has a depth of `parentsDepth + 1`.
- **Subtree** of a tree is a node and all its descendants
- A **"complete" binary tree** is one where every level (except possibly the last) is full, and all the nodes in the last level are as left as possible.

## What is a binary tree?

- A collection of nodes.
- Every node has between 0 to 2 children (left child and the right child).
- Every node except the root has exactly one parent.

    <img style="max-height: 400px" src="https://media.geeksforgeeks.org/wp-content/uploads/20200219144238/General-Tree-vs-Binary-Tree.png"/>

## Binary tree code representation

### Rust
```Rust
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
```

### Typescript
```Typescript
class TreeNode {
    val: number
    left: TreeNode | null
    right: TreeNode | null
    constructor(
        val?: number, 
        left?: ListNode | null, 
        right?: ListNode | null
    ) {
        this.val = (val === undefined ? 0 : val)
        this.left = (left === undefined ? null : left)
        this.right = (right === undefined ? null : right)
    }
}
```

### Python
```Python
from typing import Optional

class TreeNode:
    def __init__(
        self, 
        val: int = 0, 
        left: Optional['TreeNode'] = None, 
        right: Optional['TreeNode'] = None
    ) -> None:
        self.val = val
        self.left = left
        self.right = right
```

## Traverse binary trees

### Depth-first search (DFS)

- Prioritize depth by traversing as far down the tree as possible in one direction (until reaching a leaf node) before considering the other direction.

    <img style="max-height: 400px" src="https://upload.wikimedia.org/wikipedia/commons/7/7f/Depth-First-Search.gif"/>

### Rust recursive implementation
```Rust
    pub fn dfs(root: &Option<Rc<RefCell<TreeNode>>>) {
      if let Some(node) = root {
          let borrowed = node.borrow();
          Self::dfs(&borrowed.left); // 1
          Self::dfs(&borrowed.right); // 2
      }
    }
```
### Rust iterative implementation
```Rust
pub fn dfs(root: &Option<Rc<RefCell<TreeNode>>>) {
    let mut stack = vec![];
    if let Some(node) = root {
        stack.push(Rc::clone(node));
    }
    while let Some(node) = stack.pop() {
        let node = node.borrow();
        for i in 0..=1 {
            if let Some(leaf) = &node[i] {
                stack.push(Rc::clone(leaf));
            }
        }
    }
}
```
There are three types of DFS. Each of the three types differs only in the order that they execute steps 2/3. Let's use the following tree as reference:

```code
      0
    /   \
   1     2
 /   \     \
3     4     5
        \
         6
```

#### 1. Preorder traversal

- The logic is done on the current node before moving to the children. 
- Let's say that we wanted to just print the value of each node in the tree to the console. In that case, at any given node, we would print the current node's value, then recursively call the left child, then recursively call the right child.

  ```Rust
  pub fn preorder_dfs(root: &Option<Rc<RefCell<TreeNode>>>) {
      if let Some(node) = root {
          let borrowed = node.borrow();
          println!("{:?}", &borrowed.val); // preorder logic
          Self::preorder_dfs(&borrowed.left); // 1
          Self::preorder_dfs(&borrowed.right); // 2
      }
  }
  ```

- Print sequence: `0, 1, 3, 4, 6, 2, 5`

#### 2. Inorder traversal

- First recursively call the left child.
- Then perform logic on the current node, then recursively call the right child
- This means no logic will be done until we reach a node without a left child since calling on the left child takes priority over performing logic.

  ```Rust
  pub fn inorder_dfs(root: &Option<Rc<RefCell<TreeNode>>>) {
      if let Some(node) = root {
          let borrowed = node.borrow();
          Self::inorder_dfs(&borrowed.left); // 1
          println!("{:?}", &borrowed.val); // inorder logic
          Self::inorder_dfs(&borrowed.right); // 2
      }
  }
  ```
- Print sequence: `3, 1, 4, 6, 0, 2, 5`

#### 3. Postorder traversal

- First recursively call on the children first and then perform logic on the current node. 
- This means no logic will be done until we reach a leaf node since calling on the children takes priority over performing logic. 
- In a postorder traversal, the root is the last node where logic is done.

  ```Rust
  pub fn postorder_dfs(root: &Option<Rc<RefCell<TreeNode>>>) {
      if let Some(node) = root {
          let borrowed = node.borrow();
          Self::postorder_dfs(&borrowed.left); // 1
          Self::postorder_dfs(&borrowed.right); // 2
          println!("{:?}", &borrowed.val); // postorder logic
      }
  }
  ```
- Print sequence: `3, 6, 4, 1, 5, 2, 0`

### Breadth-first search (BFS)
    
<img style="max-height: 400px" src="https://upload.wikimedia.org/wikipedia/commons/5/5d/Breadth-First-Search-Algorithm.gif?20100504223639"/>

- In breadth-first search (BFS), we prioritize breadth.
- In BFS, we traverse all nodes at a given depth before moving on to the next depth. 
- So if you performed BFS on a large [complete binary tree](#terminology), the depth of the nodes you would traverse would look like `0, 1, 1, 2, 2, 2, 2, 3, 3, ...`
- While DFS was implemented using a stack (recursion uses a stack under the hood), usually **BFS is implemented iteratively with a queue**.

### Rust iterative implementation

```Rust
pub fn bfs_it(root: &Option<Rc<RefCell<TreeNode>>>) {
    let mut q = VecDeque::new();
    if let Some(node) = root {
        q.push_back(Rc::clone(&node));
    }

    while let Some(node) = q.pop_front() {
        let node = node.borrow();
        // do some logic here on the current node
        println!("{}", node.val);

        // put the next level onto the queue
        if let Some(left) = node.left.as_ref() {
            q.push_back(Rc::clone(left));
        }
        if let Some(right) = node.right.as_ref() {
            q.push_back(Rc::clone(right));
        }
    }
}
```

- With an efficient queue, the dequeue and enqueue operations are $O(1)$, which means that the time complexity of BFS is the same as DFS. 
- The main idea is that we visit each node only once, so the time complexity is $O(n*k)$ where 
$n$ is the total number of nodes, and $k$ is the amount of work we do at each node, usually 
$O(1)$.
  
### When to use BFS vs DFS?

- It is very rare to find a problem that using DFS is "better" than using BFS. However, implementing DFS is usually quicker because it requires less code, and is easier to implement if using recursion.
- Using BFS makes way more sense algorithmically than using DFS usually when we want to handle the nodes according to their level.
- **The main disadvantage of DFS** is that you could end up wasting a lot of time looking for a value. 
- **The main disadvantage of BFS** is that if the node you're searching for is near the bottom, then you will waste a lot of time searching through all the levels to reach the bottom.

#### Complexity
- If you have a [complete binary tree](#terminology), then the amount of space used by the recursive call stack for DFS is linear with the height, which is logarithmic with n (the number of nodes). The amount of space used by the queue is linear with n, **so DFS has a much better space complexity**. The reason the queue will grow linearly is because the final level in a complete binary tree can have up to $n \over 2$ nodes.
- If you have a lopsided tree (like a straight line), then BFS will have an $O(1)$ space complexity while DFS will have $O(n)$ (although, a lopsided tree is an edge case whereas a full tree is the expectation).

## Nodes, pointers, mutability (in Rust)

### Rc module-level documentation
[https://doc.rust-lang.org/std/rc/index.html](https://doc.rust-lang.org/std/rc/index.html)

### When to choose interior mutability
[https://doc.rust-lang.org/std/cell/index.html#when-to-choose-interior-mutability](https://doc.rust-lang.org/std/cell/index.html#when-to-choose-interior-mutability)

