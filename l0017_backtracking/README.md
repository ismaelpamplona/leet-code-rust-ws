# Backtracking

- Backtracking is an optimization that involves abandoning a "path" once it is determined that the path cannot lead to a solution. 
- Abandoning a path is also sometimes called **"pruning"**.
- The idea is similar to binary search trees - if you're looking for a value x, and the root node has a value greater than x, then you know you can ignore the entire right subtree. 
- Because the number of nodes in each subtree is exponential relative to the depth, backtracking can save huge amounts of computation. 
- In an exhaustive search, we generate all possibilities and then check them for solutions. **In backtracking, we prune paths that cannot lead to a solution**, generating far fewer possibilities.
- Backtracking is a great tool whenever a **problem wants you to find all of something**, or there isn't a clear way to find a solution without checking all logical possibilities. 
- A **strong hint that you should use backtracking is if the input constraints are very small** ($n <= ~15$), as backtracking algorithms usually have exponential time complexities.

## Implementation

### Pseudocode

1. **Function Definition**: The function backtrack is defined with a parameter curr. This parameter represents the current state or solution that is being built.

2. **Base Case**: The base case checks if the current state curr is a valid solution. If it is, the solution is added to the list of solutions or processed in some way.

3. **Iteration**: The function iterates over the input (or possible choices) to try and extend the current solution.

4. **Modification**: The current state curr is modified based on the current iteration. This is essentially trying out a choice.

5. **Recursive Call**: The function calls itself recursively with the modified state to further extend the solution.

6. **Backtrack**: After the recursive call returns, the modification done to the current state curr is undone. This step is crucial as it ensures that the state is reset to its original form before the next iteration. This is the actual "backtracking" step.

```
// let curr represent the thing you are building
// it could be an array or a combination of variables

function backtrack(curr) {
    if (base case) {
        Increment or add to answer
        return
    }

    for (iterate over input) {
        Modify curr
        backtrack(curr)
        Undo whatever modification was done to curr
    }
}
```

- Let's think back to the analogy of possibilities being represented by a tree.
- Each call to the function backtrack represents a node in the tree. Each iteration in the for loop represents a child of the current node, and calling backtrack in that loop represents moving to a child.
- The line where you undo the modifications is the "backtracking" step and is equivalent to moving back up the tree from a child to its parent.
- At any given node, the path from the root to the node represents a candidate that is being built. The leaf nodes are complete solutions and represent when the base case is reached. The root of this tree is an empty candidate and represents the scope that the original backtrack call is being made from.


