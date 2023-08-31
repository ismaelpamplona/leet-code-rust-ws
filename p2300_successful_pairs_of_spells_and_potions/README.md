# p2300_successful_pairs_of_spells_and_potions
[https://leetcode.com/problems/successful-pairs-of-spells-and-potions/](https://leetcode.com/problems/successful-pairs-of-spells-and-potions/)

## Initial provided code
```Rust
impl Solution {
    pub fn successful_pairs(spells: Vec<i32>, potions: Vec<i32>, success: i64) -> Vec<i32> {
        
    }
}
```
## First approach - Binary Search

- `n` = `spells.len()`
- `m` = `potions.len()`
### time complexity: $O(m \log m)$
- We iterate $O(\log m)$ binary search on each iteration. 
- This gives us a time complexity of $O((m + n) \cdot \log m)$, which is much faster than $O(m \cdot n)$ because logm is small.

### space complexity: $O(\log n)$

- In Rust, when you use the sort() method on a `Vec<i32>``, it sorts the vector in-place using an algorithm called **introsort**, which is a hybrid sorting algorithm that **combines quicksort, heapsort, and insertion sort**. The space complexity of this operation is $O(\log n)$, where 'n' is the number of elements in the vector.
- The reason for the $O(\log n)$ space complexity is because introsort uses recursion for the quicksort part of the algorithm. Each recursive call consumes stack space, and the maximum depth of the recursion is limited to $O(\log n)$ because at each step, it divides the array in half.
- So, even though the sorting operation itself doesn't allocate additional memory for a new sorted vector, it uses stack space for the recursive calls, which contributes to the $O(\log n)$ space complexity.

