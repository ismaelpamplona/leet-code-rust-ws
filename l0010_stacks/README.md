# Stacks
- A stack is an ordered collection of elements where elements are only added and removed from the same end
- Another term used to describe stacks is **LIFO**, which stands for **last in, first out**. The last (most recent) element placed inside is the first element to come out.

## Time Complexity
- Dependes on the implementation. 
- If you use a dynamic array, which is the most common and easiest way, then the time complexity of your operations is the same as that of a dynamic array:
  - $O(1)$: push, pop, and random access;
  - $O(n)$: search.

## Stacks and Recursion

- Recursion is actually done using a stack. 
- Function calls are pushed on a stack. 
- The call at the top of the stack at any given moment is the "active" call. 
- On a return statement or the end of the function being reached, the current call is popped off the stack.
