# p2294_partition_array_such_that_maximum_difference_isK
[https://leetcode.com/problems/partition-array-such-that-maximum-difference-is-k/](https://leetcode.com/problems/partition-array-such-that-maximum-difference-is-k/)

## Initial provided code
```Rust
impl Solution {
    pub fn partition_array(nums: Vec<i32>, k: i32) -> i32 {
        
    }
}
```

## First approach - Greedy

`n`: number of elements
 
### time complexity: $O(n \cdot \log n)$
- To summarize: set x at the start and take as many elements as you can. Once you go beyond x + k, increment the answer and start again with a new x. This runs in $O(n \cdot \log n)$ where `n` is the length of the input array due to the sort. 
- The time complexity of the sort() method in Rust, which uses an introsort algorithm, is $O(n \cdot \log n)$ in the average and worst cases.

### space complexity: $O(\log n)$

- In Rust, when you use the sort() method on a Vec<i32>, it sorts the vector in-place using an algorithm called **introsort**, which is a hybrid sorting algorithm that **combines quicksort, heapsort, and insertion sort**. The space complexity of this operation is $O(\log n)$, where 'n' is the number of elements in the vector.
- The reason for the $O(\log n)$ space complexity is because introsort uses recursion for the quicksort part of the algorithm. Each recursive call consumes stack space, and the maximum depth of the recursion is limited to $O(\log n)$ because at each step, it divides the array in half.
- So, even though the sorting operation itself doesn't allocate additional memory for a new sorted vector, it uses stack space for the recursive calls, which contributes to the $O(\log n)$ space complexity.






