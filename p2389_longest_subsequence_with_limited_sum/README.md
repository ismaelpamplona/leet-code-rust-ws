# p2389_longest_subsequence_with_limited_sum
[https://leetcode.com/problems/longest-subsequence-with-limited-sum/](https://leetcode.com/problems/longest-subsequence-with-limited-sum/)

## Initial provided code
```Rust
impl Solution {
    pub fn answer_queries(nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {
        
    }
}
```
  
## First approach - Sort and count

- `n` = `nums.len()`
- `m` = `queries.len()`
  
### time complexity: $O(m \cdot n + n \cdot \log n)$
- Sorting `nums`: $O(n \cdot log n)$.
- For each query, we need to iterate over the sorted nums to find the longest subsequence, which takes $O(n)$ in the worst-case scenario, so mmm queries take $O(n \cdot m)$ time.
Therefore, the overall time complexity is O(m⋅n+n⋅log⁡n)O(m\cdot n + n\cdot \log n)O(m⋅n+n⋅logn).

### space complexity: $O(\log n)$

- In Rust, when you use the sort() method on a `Vec<i32>``, it sorts the vector in-place using an algorithm called **introsort**, which is a hybrid sorting algorithm that **combines quicksort, heapsort, and insertion sort**. The space complexity of this operation is $O(\log n)$, where 'n' is the number of elements in the vector.
- The reason for the $O(\log n)$ space complexity is because introsort uses recursion for the quicksort part of the algorithm. Each recursive call consumes stack space, and the maximum depth of the recursion is limited to $O(\log n)$ because at each step, it divides the array in half.
- So, even though the sorting operation itself doesn't allocate additional memory for a new sorted vector, it uses stack space for the recursive calls, which contributes to the $O(\log n)$ space complexity.

## Second approach - Prefix Sum + Binary Search

- `n` = `nums.len()`
- `m` = `queries.len()`
  
### time complexity: $O((m + n) \cdot \log n)$
- Sorting `nums`: $O(n \cdot log n)$.
- Building `presum`: $O(n)$.
- For each query, binary search over the prefix sum array to find its insertion index, it takes $O(\log n)$. There are mmm queries so the time required is $O(m \cdot \log n)$.
- Therefore, the overall time complexity is $O(n \cdot \log n + m \cdot \log n) = O((m + n) \cdot n)$.

### space complexity: $O(\log n)$

- Same above.

