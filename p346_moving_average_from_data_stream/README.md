# p346_moving_average_from_data_stream
[https://leetcode.com/problems/moving-average-from-data-stream/](https://leetcode.com/problems/moving-average-from-data-stream/)

## Initial provided code
```Rust
struct MovingAverage {

}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MovingAverage {

    fn new(size: i32) -> Self {
        
    }
    
    fn next(&self, val: i32) -> f64 {
        
    }
}

/**
 * Your MovingAverage object will be instantiated and called as such:
 * let obj = MovingAverage::new(size);
 * let ret_1: f64 = obj.next(val);
 */
```

## First approach - queue

- k = length of the window
- time complexity: O(n)
- space complexity: O(k)
