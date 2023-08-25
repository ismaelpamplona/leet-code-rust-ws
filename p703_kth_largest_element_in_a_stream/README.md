# p703_kth_largest_element_in_a_stream
[https://leetcode.com/problems/kth-largest-element-in-a-stream/](https://leetcode.com/problems/kth-largest-element-in-a-stream/)

## Initial provided code
```Rust
struct KthLargest {

}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl KthLargest {

    fn new(k: i32, nums: Vec<i32>) -> Self {
        
    }
    
    fn add(&self, val: i32) -> i32 {
        
    }
}

/**
 * Your KthLargest object will be instantiated and called as such:
 * let obj = KthLargest::new(k, nums);
 * let ret_1: i32 = obj.add(val);
 */
```

## First approach - Heap

### time complexity: $O(n \cdot \log(n) + m \cdot \log(k))$
#### Constructor time complexity

1. First, the constructor needs to turn nums into a heap of size k $O(n)$.
1. Then, we need to remove from the heap until there are only k elements in it, which means removing $n - k$ elements. 
1. Since k can be, say 1, in terms of big $O$ this is `n` operations, with each operation costing $log(n)$. 
1. Therefore, the constructor costs $O(n + n \cdot \log(n)) = O(n \cdot \log(n))$.

#### `add()` method time complexity

- Every call to `add()`` involves adding an element to heap and potentially removing an element from heap. 
- Since our heap is of size k, every call to `add()` at worst costs $O(2∗log(k))=O(log(k))$. 
- That means $m$ calls to `add()` costs $O(m \cdot \log(k))$.
  
### space complexity: $O(n)$

- The only extra space we use is the heap. 
- While during `add()` calls we limit the size of the heap to $k$, in the constructor we start by converting `nums into a heap, which means the heap will initially be of size $n$.


