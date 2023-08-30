# p881_boats_to_save_people
[https://leetcode.com/problems/boats-to-save-people/](https://leetcode.com/problems/boats-to-save-people/)

## Initial provided code
```Rust
impl Solution {
    pub fn num_rescue_boats(people: Vec<i32>, limit: i32) -> i32 {
        
    }
}
```

## First approach - Greedy + two pointers

`n`: number of people
 
### time complexity: $O(n \cdot \log n)$
- The two pointers part of the algorithm runs in $O(n)$.
- Sort part: $O(\log n)$.
  
### space complexity: $O(\log n)$
- In Rust, when you use the sort() method on a Vec<i32>, it sorts the vector in-place using an algorithm called **introsort**, which is a hybrid sorting algorithm that **combines quicksort, heapsort, and insertion sort**. The space complexity of this operation is $O(\log n)$, where 'n' is the number of elements in the vector.
- The reason for the $O(\log n)$ space complexity is because introsort uses recursion for the quicksort part of the algorithm. Each recursive call consumes stack space, and the maximum depth of the recursion is limited to $O(\log n)$ because at each step, it divides the array in half.
- So, even though the sorting operation itself doesn't allocate additional memory for a new sorted vector, it uses stack space for the recursive calls, which contributes to the $O(\log n)$ space complexity.






