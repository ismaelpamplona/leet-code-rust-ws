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

## Interface guide

### Rust
```Rust
let mut stack: Vec<i32> = Vec::new();

stack.push(1);
stack.push(2);
stack.push(3);

stack.pop(); // Some(3)
stack.pop(); // Some(2)

stack.is_empty(); // false

stack.last(); // Some(&1)

stack.len(); // 1
```

### C++
```c++
stack<int> stack;

stack.push(1);
stack.push(2);
stack.push(3);

// Unlike other languages, popping here does not return the popped value
stack.pop();
stack.pop();

stack.empty(); // false

stack.top(); // 1

stack.size(); // 1
```

### Typescript
```Typescript
let stack: number[] = [];

stack.push(1);
stack.push(2);
stack.push(3);

stack.pop(); // 3
stack.pop(); // 2

!stack.length; // false

stack[stack.length - 1]; // 1

stack.length; // 1
```

### Python
```Python
stack = []

# Pushing elements:
stack.append(1)
stack.append(2)
stack.append(3)

stack.pop() # 3
stack.pop() # 2

not stack # False

stack[-1] # 1 - Check element at top

len(stack) # 1
```
