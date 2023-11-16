# p2140_solving_questions_with_brainpower

[https://leetcode.com/problems/solving-questions-with-brainpower/](https://leetcode.com/problems/solving-questions-with-brainpower/)

## Initial provided code

```Rust
impl Solution {
    pub fn most_points(questions: Vec<Vec<i32>>) -> i64 {

    }
}
```

## First approach - Dynamic programming (recursive top down)

### Time Complexity: $O(n)$

- **States**: There are $n$ unique states, each corresponding to an index in the questions array.
- **Transitions**: Each state has a constant-time transition.
- **Overall**: The algorithm iterates through $n$ questions once, leading to a linear time complexity.

### Space Complexity: $O(n)$

- **Memoization**: Uses a `HashMap` or a `Vec` for storing computed results for $n$ questions.
- **Call Stack**: Negligible compared to memoization storage.
- **Overall**: Primarily dominated by the memoization storage, resulting in a linear space complexity.

## Second approach - Dynamic programming (bottom up)

### Time Complexity: $O(n)$

- **Iteration**: The algorithm iterates backward through the array of questions, which has $n$ elements.
- **HashMap Operations**: Each iteration involves constant-time operations (insertion and retrieval) in the `HashMap`.
- **Overall**: The combination of iterating through $n$ elements and constant-time `HashMap` operations results in a linear time complexity.

### Space Complexity: $O(n)$

- **HashMap Storage**: The `HashMap` is used to store computed results for each question. There are $n$ questions, leading to $n$ entries in the `HashMap`.
- **Overall**: Since the space used is proportional to the number of questions, the space complexity is linear.
