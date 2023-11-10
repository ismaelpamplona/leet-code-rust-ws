# Dynamic Programming

Dynamic programming is a method for solving complex problems by breaking them down into simpler subproblems. It is applicable in various fields, such as mathematics, computer science, economics, and engineering. The concept is based on two main principles:

## 1. Optimal Substructure

This principle states that a problem can be broken down into smaller, simpler subproblems which can be solved independently. The solutions to these smaller problems are then combined to solve the original problem. For dynamic programming to be effective, the problem must exhibit optimal substructure.

## 2. Overlapping Subproblems

Dynamic programming is particularly useful when a problem has many overlapping subproblems. These are smaller parts of the problem that are solved multiple times during the computation. Dynamic programming saves time by caching the results of these subproblems and reusing them.

## Applications

Dynamic programming is used in various scenarios, from optimizing resource allocation to solving complex algorithmic problems like shortest path in graph theory, the knapsack problem, Fibonacci number computation, and many others. It's a powerful technique for improving the efficiency of algorithms dealing with complex, multi-step problems.

### Fibonacci

The time complexity of a straightforward recursive Fibonacci algorithm is $O(2^n)$. This complexity arises because each call to the `fibonacci` function creates two more calls to `fibonacci`. Below is the recursion tree for `fibonacci(6)` to illustrate this:

#### Recursion Tree for `fibonacci(6)`:

```scss
             fn(6)
           /       \
      fn(5)         fn(4)
     /     \       /     \
 fn(4)   fn(3)   fn(3)   fn(2)
/    \   /    \   /   \   /   \
```

- Each node in this tree represents a function call, and the tree branches out exponentially with the height of the tree.
- The height of the tree is `n`, and at each level, the number of nodes doubles.
- Therefore, the total number of nodes (function calls) in this tree, which represents the total amount of work done, is proportional to $2^n$.

This exponential growth in the number of function calls is what leads to the time complexity of $O(2^n)$ for the naive recursive implementation of the Fibonacci sequence.

When analyzing the recursion tree for the Fibonacci sequence, it becomes evident that there is a significant amount of repeated computation. For example, consider the recursion tree for $fn(6)$:

In this structure:

- `fn(4)` is calculated twice.
- `fn(3)` is calculated three times.
- `fn(2)` is calculated five times.

At a smaller scale, like in $fn(6)$, this may not seem like a significant issue. However, the problem becomes increasingly severe as `n` grows. The number of repeated calculations grows exponentially with `n`. To illustrate, if we were to calculate $fn(7)$, then the entire tree shown above would represent just one branch (the left side) of the root node for $fn(7)$.

This exponential growth in repeated computation highlights a major inefficiency in the naive recursive approach to computing Fibonacci numbers.

```rust
fn fibonacci(n: i32) -> i32 {
    if n <= 1 {
        return n;
    } else {
        let one_back = return_nth_fibo_number(n - 1);
        let two_back = return_nth_fibo_number(n - 2);
        return one_back + two_back;
    }
}
```

To avoid repeating computation, we can memoize the results from our function calls. Let's use a hash map to store the results and check the hash map before making any recursive calls.

```rust
fn fibonacci(n: i32, map: &mut HashMap<i32, i32>) -> i32 {
    if n <= 1 {
        return n;
    } else if let Some(num) = map.get(&n) {
        return *num;
    } else {
        let one_back = fibonacci(n - 1, map);
        let two_back = fibonacci(n - 2, map);
        return one_back + two_back;
    }
}
```

This improves our time complexity to $O(n)$ - which is, of course, extremely fast compared to $O(n)$. The first approach is just basic recursion - **by memoizing results to avoid duplicate computation, it becomes dynamic programming**.

## Top-Down Approach (Memoization)

- This approach starts with the original problem and recursively breaks it down into subproblems.
- As each subproblem is solved, its result is stored in a data structure, such as an array or a hash table.
- Before solving a subproblem, the method checks if the solution is already known, thereby avoiding recalculations.

## Bottom-Up Approach (Tabulation)

- This approach involves solving all small subproblems first and then combining these solutions to solve larger subproblems.
- This is typically done using iterative methods and storing the results of subproblems in a table (array).
